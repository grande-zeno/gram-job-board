use super::{BotResult, EditDialogue};
use teloxide::prelude::*;
use crate::SharedState;

pub async fn update_company_name(
    bot: Bot,
    codename: String,
    name: String,
    dialogue: EditDialogue,
    state: SharedState,
    msg: Message
) -> BotResult {
    let query = sqlx::query("update jobs set company_name = $1 where job_id = $2 and user_id = $3")
    .bind(name)
    .bind(codename)
    .bind(msg.chat.id.0.to_string())
    .execute(&state.pool)
    .await;

    match query {
        Ok(_) => {
            bot.send_message(msg.chat.id, "Company name updated successfully.\nType /myjobs to get all jobs").await?;
        },
        Err(_) => {
            bot.send_message(msg.chat.id, "Something went wrong.\nPlease Try again!").await?;
        }

    }
    dialogue.exit().await?;

    Ok(())
}
