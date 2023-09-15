use axum::{
    extract::{Extension, Multipart},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use bytes::Bytes;
use futures::{StreamExt, TryStreamExt};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, env, io::Cursor};
use tokio::io::AsyncWriteExt;
use chrono;

// Struct to hold S3 credentials
#[derive(Debug, Serialize, Deserialize)]
struct S3Credentials {
    access_key: String,
    secret_key: String,
    bucket: String,
}

// Function to upload a file to S3
async fn upload_to_s3(
    credentials: Extension<S3Credentials>,
    file_name: String,
    file_data: Bytes,
) -> Result<Response, reqwest::Error> {
    // Set up headers for S3 PUT request
    let content_length = file_data.len();
    let content_type = mime_guess::from_path(&file_name).first_or_octet_stream().to_string();
    let date = chrono::Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();
    let host = format!("{}.s3.amazonaws.com", credentials.bucket);
    let path = format!("/{}", file_name);
    let string_to_sign = format!(
        "PUT\n\n{}\n{}\n/{bucket}/{}",
        content_type, date, file_name, bucket = credentials.bucket
    );
    let signature = {
        let secret_key = credentials.secret_key.as_bytes();
        let signing_key = hmacsha1::hmac_sha1(secret_key, date.as_bytes());
        base64::encode(hmacsha1::hmac_sha1(&signing_key, string_to_sign.as_bytes()))
    };
    let url = format!("https://{}.s3.amazonaws.com/{}", credentials.bucket, file_name);

    // Send PUT request to S3
    let client = Client::new();
    let response = client
        .put(&url)
        .header("Content-Length", content_length)
        .header("Content-Type", content_type)
        .header("Date", date)
        .header("Authorization", format!("AWS {}:{}", credentials.access_key, signature))
        .header("Host", host)
        .body(Cursor::new(file_data))
        .send()
        .await?;

    Ok(response)
}

// Trait for handling form fields
trait FieldHandler {
    fn is_match(&self, content_disposition: &str) -> bool;
    fn handle(
        &self,
        part: axum::extract::Part,
        field_value: &mut Option<String>,
        logo_name: &mut Option<String>,
        logo_data: &mut Option<Bytes>,
    ) -> Result<(), StatusCode>;
}

// Implementation of FieldHandler for text fields
struct TextFieldHandler {
    name: &'static str,
}

impl FieldHandler for TextFieldHandler {
    fn is_match(&self, content_disposition: &str) -> bool {
        content_disposition.contains(&format!("name=\"{}\"", self.name))
    }

    async fn handle(
        &self,
        mut part: axum::extract::Part,
        field_value: &mut Option<String>,
        _logo_name: &mut Option<String>,
        _logo_data: &mut Option<Bytes>,
    ) -> Result<(), StatusCode> {
        let mut data = Vec::new();
        while let Some(chunk) = part.next().await {
            let chunk = chunk.map_err(|_| StatusCode::BAD_REQUEST)?;
            data.extend_from_slice(&chunk);
        }
        *field_value = Some(String::from_utf8(data).map_err(|_| StatusCode::BAD_REQUEST)?);
        Ok(())
    }
}

// Implementation of FieldHandler for file fields
struct FileFieldHandler;

impl FieldHandler for FileFieldHandler {
    fn is_match(&self, content_disposition: &str) -> bool {
        content_disposition.contains("filename=")
    }

    async fn handle(
        &self,
        mut part: axum::extract::Part,
        _field_value: &mut Option<String>,
        logo_name: &mut Option<String>,
        logo_data: &mut Option<Bytes>,
    ) -> Result<(), StatusCode> {
        let content_disposition = part.headers().get("content-disposition");
        if let Some(content_disposition) = content_disposition {
            let content_disposition = content_disposition.to_str().map_err(|_| StatusCode::BAD_REQUEST)?;
            let file_name = content_disposition
                .split("filename=")
                .nth(1)
                .ok_or(StatusCode::BAD_REQUEST)?
                .trim_matches('"')
                .to_owned();
            let mut data = Vec::new();
            while let Some(chunk) = part.next().await {
                let chunk = chunk.map_err(|_| StatusCode::BAD_REQUEST)?;
                data.extend_from_slice(&chunk);
            }
            *logo_name = Some(file_name);
            *logo_data = Some(Bytes::from(data));
        }
        Ok(())
    }
}

// Function to handle a single form field
async fn handle_field(
    part: axum::extract::Part,
    field_handlers: &[Box<dyn FieldHandler>],
    field_value: &mut Option<String>,
    logo_name: &mut Option<String>,
    logo_data: &mut Option<Bytes>,
) -> Result<(), StatusCode> {
    let content_disposition = part.headers().get("content-disposition");
    if let Some(content_disposition) = content_disposition {
        let content_disposition = content_disposition.to_str().map_err(|_| StatusCode::BAD_REQUEST)?;
        for handler in field_handlers {
            if handler.is_match(content_disposition) {
                handler.handle(part, field_value, logo_name, logo_data)?;
                break;
            }
        }
    }
    Ok(())
}

// Handler function for file upload
async fn handle_upload(
    Multipart(mut parts): Multipart,
    credentials: Extension<S3Credentials>,
) -> Result<impl IntoResponse, StatusCode> {
    // Initialize variables to hold form data
    let mut name = None;
    let mut description = None;
    let mut website = None;
    let mut logo_name = None;
    let mut logo_data = None;

    // Define field handlers
    let field_handlers: Vec<Box<dyn FieldHandler>> = vec![
        Box::new(TextFieldHandler { name: "name" }),
        Box::new(TextFieldHandler { name: "description" }),
        Box::new(TextFieldHandler { name: "website" }),
        Box::new(FileFieldHandler),
    ];

    // Iterate over form data parts and handle each field
    while let Some(part) = parts.try_next().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        handle_field(part, &field_handlers, &mut name, &mut logo_name, &mut logo_data).await?;
    }

    // Ensure all form fields were present
    let name = name.ok_or(StatusCode::BAD_REQUEST)?;
    let description = description.ok_or(StatusCode::BAD_REQUEST)?;
    let website = website.ok_or(StatusCode::BAD_REQUEST)?;
    let logo_name = logo_name.ok_or(StatusCode::BAD_REQUEST)?;
    let logo_data = logo_data.ok_or(StatusCode::BAD_REQUEST)?;

    // Upload logo to S3
    upload_to_s3(credentials, logo_name.clone(), logo_data)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Return response body
    let response_body = format!(
        "Created company {} with description {} and website {} and logo {}",
        name, description, website, logo_name
    );
    Ok(response_body)
}