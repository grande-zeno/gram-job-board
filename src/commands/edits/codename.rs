use super::{BotResult, EditDialogue, EditState};
use crate::SharedState;
use teloxide::prelude::*;

pub async fn receive_codename(bot: Bot, dialogue: EditDialogue, msg: Message) -> BotResult {
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, format!("So what do you want to edit
            \nType one of the following to edit the field of your choosing\nCompany name\nLocation\nSalary\nTitle")).await?;

            dialogue
                .update(EditState::CheckPoint {
                    codename: text.into(),
                })
                .await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }

    Ok(())
}
