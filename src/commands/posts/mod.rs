pub mod company;
pub mod location;
pub mod salary;
pub mod start;
pub mod title;

pub use super::BotResult;
pub use start::start;

use teloxide::{
    dispatching::{dialogue::InMemStorage, DpHandlerDescription},
    prelude::*,
};

pub type PostDialogue = Dialogue<State, InMemStorage<State>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveCompanyName,
    ReceiveTitle {
        company_name: String,
    },
    ReceiveSalaryRange {
        company_name: String,
        title: String,
    },
    ReceiveLocation {
        company_name: String,
        title: String,
        salary_range: String,
    },
}

#[derive(sqlx::FromRow)]
pub struct Job {
    pub job_id: String,
    pub company_name: String,
    pub location: String,
    pub salary_range: String,
    pub job_title: String,
}
pub fn post_updates() -> Handler<'static, DependencyMap, BotResult, DpHandlerDescription> {
    Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<State>, State>()
        .branch(dptree::case![State::ReceiveCompanyName].endpoint(company::receive_company_name))
        .branch(
            dptree::case![State::ReceiveTitle { company_name }].endpoint(title::receive_job_title),
        )
        .branch(
            dptree::case![State::ReceiveSalaryRange {
                company_name,
                title
            }]
            .endpoint(salary::receive_salary_range),
        )
        .branch(
            dptree::case![State::ReceiveLocation {
                company_name,
                title,
                salary_range
            }]
            .endpoint(location::receive_location),
        )
}
