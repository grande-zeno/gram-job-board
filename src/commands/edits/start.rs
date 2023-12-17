use super::{BotResult, EditDialogue, EditState};
use teloxide::prelude::*;

pub async fn start(bot: Bot, dialogue: EditDialogue, msg: Message) -> BotResult {
    bot.send_message(msg.chat.id, "Here We Go!\nType the code name of the job you want to edit\nOnce you're satisfied type quit to exit the dialogue").await?;
    dialogue.update(EditState::ReceiveCodeName).await?;

    Ok(())
}
