use askama::Template;

use crate::models::{
    applicant::Applicant,
    application::Application,
    company::{Company, CompanyIdName},
    job::{Job, JobLocation, JobWithCompany},
};

#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloTemplate<'a> {
    pub name: &'a str,
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub companies: Vec<CompanyIdName>,
    pub locations: Vec<JobLocation>,
    pub jobs: Vec<JobWithCompany>,
}

#[derive(Template)]
#[template(path = "partials/list_jobs.html")]
pub struct ListJobsTemplate {
    pub jobs: Vec<JobWithCompany>,
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
