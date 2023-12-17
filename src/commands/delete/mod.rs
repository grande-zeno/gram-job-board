use super::BotResult;
use crate::SharedState;
use teloxide::prelude::*;

pub async fn delete_job(
    code_name: String,
    bot: Bot,
    msg: Message,
    state: SharedState,
) -> BotResult {
    let query = sqlx::query("delete from jobs where user_id = $1 and job_id = $2")
        .bind(msg.chat.id.to_string())
        .bind(code_name)
        .execute(&state.pool)
        .await;

    match query {
        Ok(_) => {
            bot.send_message(
                msg.chat.id,
                format!("\nJob deleted successfully\nType /myjobs so see all remaining jobs"),
            )
            .await?;
        }
        Err(_) => {
            bot.send_message(
                msg.chat.id,
                format!("\nYou don't have any jobs with this name."),
            )
            .await?;
        }
    }
    Ok(())
}
