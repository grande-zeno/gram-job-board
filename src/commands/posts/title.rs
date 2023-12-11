use super::{BotResult, PostDialogue, State};
use teloxide::prelude::*;

pub async fn receive_job_title(
    bot: Bot,
    dialogue: PostDialogue,
    company_name: String,
    msg: Message,
) -> BotResult {
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, "What about the salary range?")
                .await?;
            dialogue
                .update(State::ReceiveSalaryRange {
                    company_name,
                    title: text.into(),
                })
                .await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }

    Ok(())
}
