use crate::SharedState;
use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate {
    jobs: Vec<Job>,
}

impl HomeTemplate {
    pub fn new(jobs: Vec<Job>) -> HomeTemplate {
        HomeTemplate { jobs }
    }
}

#[derive(sqlx::FromRow)]
struct Job {
    id: i32,
    company_name: String,
    location: String,
    salary_range: String,
    job_title: String,
}

pub async fn jobs(State(state): State<SharedState>) -> impl IntoResponse {
    let query = sqlx::query_as::<_, Job>(
        "select id, company_name, location, salary_range, job_title from jobs order by id desc",
    )
    .fetch_all(&state.pool)
    .await;

    match query {
        Ok(jobs) => {
            let template = HomeTemplate::new(jobs);

            match template.render() {
                Ok(html) => Html(html).into_response(),
                Err(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to render template. Error: {err}"),
                )
                    .into_response(),
            }
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error: {err}"),
        )
            .into_response(),
    }
}
