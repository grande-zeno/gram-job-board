use super::{BotResult, EditDialogue, EditState};
use teloxide::prelude::*;

pub async fn checkpoint(
    bot: Bot,
    dialogue: EditDialogue,
    msg: Message,
    codename: String,
) -> BotResult {
    let description = format!("\nType one of the following to edit the field of your choosing\nCompany name\nLocation\nSalary\nTitle");

    match msg.text() {
        Some(text) => match text {
            "Company name" => {
                bot.send_message(msg.chat.id, "What company name do you want to use?")
                    .await?;
                dialogue
                    .update(EditState::CmdCheckPoint {
                        codename: codename,
                        cmd: text.into(),
                    })
                    .await?;
            }
            "Location" => {
                bot.send_message(msg.chat.id, "What you like it to be?")
                    .await?;
                dialogue
                    .update(EditState::CmdCheckPoint {
                        codename: codename,
                        cmd: text.into(),
                    })
                    .await?;
            }
            "Title" => {
                bot.send_message(msg.chat.id, "What title do you want to use?")
                    .await?;
                dialogue
                    .update(EditState::CmdCheckPoint {
                        codename: codename,
                        cmd: text.into(),
                    })
                    .await?;
            }
            "Salary" => {
                bot.send_message(msg.chat.id, "How much do you want to pay your employee? 🥹")
                    .await?;
                dialogue
                    .update(EditState::CmdCheckPoint {
                        codename: codename,
                        cmd: text.into(),
                    })
                    .await?;
            }
            "quit" => {
                bot.send_message(msg.chat.id, "Exiting dialogue...").await?;
                dialogue.exit().await?;
            }
            _ => {
                bot.send_message(msg.chat.id, description).await?;
            }
        },
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }

    Ok(())
}
