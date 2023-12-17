use super::{BotResult, PostDialogue, State};
use teloxide::prelude::*;

pub async fn start(bot: Bot, dialogue: PostDialogue, msg: Message) -> BotResult {
    bot.send_message(msg.chat.id, "Let's start! What's your company name?")
        .await?;
    dialogue.update(State::ReceiveCompanyName).await?;

    Ok(())
}
