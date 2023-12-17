use super::posts::{BotResult, Job};
use crate::SharedState;
use teloxide::prelude::*;

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
                            "Code name: {}\nCompany name: {}\nTitle: {}\nSalary Range: {}\nLocation: {}\n\n",
                            job.job_id,
                            job.company_name, 
                            job.job_title, 
                            job.salary_range, 
                            job.location
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
