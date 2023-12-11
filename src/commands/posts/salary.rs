use super::{BotResult, PostDialogue, State};
use teloxide::prelude::*;

pub async fn receive_salary_range(
    bot: Bot,
    dialogue: PostDialogue,
    (company_name, title): (String, String),
    msg: Message,
) -> BotResult {
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, "Location? e.g London")
                .await?;
            dialogue
                .update(State::ReceiveLocation {
                    company_name,
                    title,
                    salary_range: text.into(),
                })
                .await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }
    Ok(())
}
