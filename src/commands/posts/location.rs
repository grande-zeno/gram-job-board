use super::{BotResult, PostDialogue};
use crate::SharedState;
use rnglib::{Language, RNG};
use teloxide::prelude::*;

pub async fn receive_location(
    bot: Bot,
    dialogue: PostDialogue,
    (company_name, title, salary_range): (String, String, String),
    msg: Message,
    state: SharedState,
) -> BotResult {
    match msg.text() {
        Some(location) => {
            let rng = RNG::try_from(&Language::Fantasy).unwrap();

            let random_job_name = rng.generate_name();

            let query = sqlx::query("INSERT INTO jobs (job_id, user_id, company_name, location, salary_range, job_title) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(random_job_name)
            .bind(msg.chat.id.0.to_string())
            .bind(&company_name)
            .bind(&location)
            .bind(&salary_range)
            .bind(&title)
            .execute(&state.pool)
            .await;

            match query {
                Ok(_) => {
                    bot.send_message(
                        msg.chat.id,
                        "Job posted successfully\nTo see all your posted jobs type /myjobs",
                    )
                    .await?;

                    let report = format!("This is what will be saved.\n\nCompany name: {company_name}\nTitle: {title}\nSalary Range: {salary_range}\nLocation: {location}");
                    bot.send_message(msg.chat.id, report).await?
                }
                Err(_) => {
                    bot.send_message(
                        msg.chat.id,
                        format!("Something went wrong! Try again!🥲\n\n"),
                    )
                    .await?
                }
            };

            dialogue.exit().await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }

    Ok(())
}
