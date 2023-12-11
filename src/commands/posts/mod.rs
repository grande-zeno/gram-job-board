pub mod company;
pub mod location;
pub mod salary;
pub mod title;

pub use super::BotResult;
pub use company::receive_company_name;
pub use location::receive_location;
pub use salary::receive_salary_range;
pub use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
pub use title::receive_job_title;

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
