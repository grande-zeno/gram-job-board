-- Add migration script here
CREATE TABLE IF NOT EXISTS jobs (
    id serial primary key,
    job_id varchar not null,
    user_id varchar not null,
    company_name varchar not null,
    location varchar not null,
    salary_range varchar not null,
    job_title varchar not null
);