use super::{
    BotResult,
    EditDialogue,
    company,
    location,
    salary,
    title
};
use teloxide::prelude::*;
use crate::SharedState;

pub async fn cmd_checkpoint(
    bot: Bot,
    msg: Message,
    dialogue: EditDialogue,
    state: SharedState,
    (codename, cmd): (String, String),
) -> BotResult {
    let description = format!("\nType one of the following to edit the field of your choosing\nCompany name\nLocation\nSalary\nTitle");

    match msg.text() {
        Some(text) => match cmd.as_str() {
            "Company name" => company::update_company_name(bot, codename, text.to_string(), dialogue, state, msg).await?,
            "Location" => location::update_location(bot, codename, text.to_string(), dialogue, state, msg).await?,
            "Title" => title::update_job_title(bot, codename, dialogue, text.to_string(), state, msg).await?,
            "Salary" => salary::update_salary_range(bot, codename, text.to_string(), dialogue, state, msg).await?,
            "quit" => {
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
