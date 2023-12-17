use super::{BotResult, EditDialogue};
use teloxide::prelude::*;
use crate::SharedState;


pub async fn update_salary_range(
    bot: Bot,
    codename: String,
    salary_range: String,
    dialogue: EditDialogue,
    state: SharedState,
    msg: Message
) -> BotResult {
    
    let query = sqlx::query("update jobs set salary_range = $1 where job_id = $2 and user_id = $3")
    .bind(salary_range)
    .bind(codename)
    .bind(msg.chat.id.0.to_string())
    .execute(&state.pool)
    .await;

    match query {
        Ok(_) => {
            bot.send_message(msg.chat.id, "Salary Range updated successfully.\nType /myjobs to get all jobs").await?;
        },
        Err(_) => {
            bot.send_message(msg.chat.id, "Something went wrong.\nPlease Try again!").await?;
        }

    }
    
    dialogue.exit().await?;

    Ok(())
}
