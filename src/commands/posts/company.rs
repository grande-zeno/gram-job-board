use super::{BotResult, PostDialogue, State};
use teloxide::prelude::*;

pub async fn receive_company_name(bot: Bot, dialogue: PostDialogue, msg: Message) -> BotResult {
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, "What's the job title?")
                .await?;
            dialogue
                .update(State::ReceiveTitle {
                    company_name: text.into(),
                })
                .await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }

    Ok(())
}
