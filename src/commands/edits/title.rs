use super::{BotResult, EditDialogue};
use teloxide::prelude::*;
use crate::SharedState;


pub async fn update_job_title(
    bot: Bot,
    codename: String,
    dialogue: EditDialogue,
    title: String,
    state: SharedState,
    msg: Message
) -> BotResult {
    
    let query = sqlx::query("update jobs set job_title = $1 where job_id = $2 and user_id = $3")
    .bind(title)
    .bind(codename)
    .bind(msg.chat.id.0.to_string())
    .execute(&state.pool)
    .await;

    match query {
        Ok(_) => {
            bot.send_message(msg.chat.id, "Job title updated successfully.\nType /myjobs to get all jobs").await?;
        },
        Err(_) => {
            bot.send_message(msg.chat.id, "Something went wrong.\nPlease Try again!").await?;
        }

    }
    dialogue.exit().await?;

    Ok(())
}
