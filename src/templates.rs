use askama::Template;

use crate::models::{job::Job, applicant::Applicant, application::Application, company::Company};

#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloTemplate<'a> {
    pub name: &'a str,
}

#[derive(Template)]
#[template(path = "company.html")]
pub struct CompanyTemplate<'a> {
    pub company: &'a Company,
}

#[derive(Template)]
#[template(path = "job.html")]
pub struct JobTemplate<'a> {
    pub job: &'a Job,
}

#[derive(Template)]
#[template(path = "applicant.html")]
pub struct ApplicantTemplate<'a> {
    pub applicant: &'a Applicant,
}

#[derive(Template)]
#[template(path = "application.html")]
pub struct ApplicationTemplate<'a> {
    pub application: &'a Application,
}