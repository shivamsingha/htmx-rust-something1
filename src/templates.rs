use askama::Template;

use crate::models::{job::Job, applicant::Applicant, application::Application, company::Company};

#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloTemplate<'a> {
    pub name: &'a str,
}

#[derive(Template)]
#[template(path = "company.html")]
pub struct CompanyTemplate {
    pub company: Company,
}

#[derive(Template)]
#[template(path = "job.html")]
pub struct JobTemplate {
    pub job: Job,
}

#[derive(Template)]
#[template(path = "applicant.html")]
pub struct ApplicantTemplate {
    pub applicant: Applicant,
}

#[derive(Template)]
#[template(path = "application.html")]
pub struct ApplicationTemplate {
    pub application: Application,
}