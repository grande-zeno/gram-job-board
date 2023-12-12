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

use crate::SharedState;

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
struct Job {
    id: i32,
    company_name: String,
    location: String,
    salary_range: String,
    job_title: String,
}

pub async fn myjobs(bot: Bot, state: SharedState, msg: Message) -> BotResult {
    let user_exists_query = sqlx::query("SELECT user_id FROM jobs WHERE user_id = $1")
        .bind(msg.chat.id.0.to_string())
        .fetch_one(&state.pool)
        .await;

    match user_exists_query {
        Ok(_) => {
            let query = sqlx::query_as::<_, Job>("SELECT * FROM jobs WHERE user_id = $1")
                .bind(msg.chat.id.0.to_string())
                .fetch_all(&state.pool)
                .await;

            match query {
                Ok(jobs) => {
                    let mut job_string = String::new();

                    for job in jobs {
                        let formatted_string = format!(
                            "Company name: {}
                        \nTitle: {}
                        \nSalary Range: {}
                        \nLocation: {}\n\n",
                            job.company_name, job.job_title, job.salary_range, job.location
                        );

                        job_string.push_str(&formatted_string);
                    }
                    bot.send_message(msg.chat.id, format!("{job_string}\n"))
                        .await?;
                }
                Err(_) => {
                    bot.send_message(
                        msg.chat.id,
                        format!("\nYou haven't posted any jobs. To start type /post"),
                    )
                    .await?;
                }
            }
        }
        Err(_) => {
            bot.send_message(
                msg.chat.id,
                format!("\nYou haven't posted any jobs. To start type /post"),
            )
            .await?;
        }
    }

    Ok(())
}
