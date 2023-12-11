pub mod posts;

use crate::SharedState;
use posts::*;
use sqlx::Row;
use std::error::Error;
use teloxide::{dispatching::dialogue::InMemStorage, utils::command::BotCommands};

pub type BotResult = Result<(), Box<dyn Error + Send + Sync>>;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "displays all commands")]
    Help,
    #[command(description = "handles posting jobs")]
    Post,
    #[command(description = "see all the jobs you posted")]
    MyJobs,
}

async fn answer(
    bot: Bot,
    msg: Message,
    cmd: Command,
    dialogue: PostDialogue,
    state: SharedState,
) -> BotResult {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Post => {
            bot.send_message(msg.chat.id, "Let's start! What's your company name?")
                .await?;
            dialogue.update(State::ReceiveCompanyName).await?;
        }
        Command::MyJobs => {
            let user_exists_query = sqlx::query("SELECT user_id FROM jobs WHERE user_id = $1")
                .bind(msg.chat.id.0.to_string())
                .fetch_one(&state.pool)
                .await;

            match user_exists_query {
                Ok(_) => {
                    let query = sqlx::query("SELECT * FROM jobs WHERE user_id = $1")
                        .bind(msg.chat.id.0.to_string())
                        .fetch_all(&state.pool)
                        .await;

                    match query {
                        Ok(pg_rows) => {
                            let mut job_string = String::new();

                            for job in pg_rows {
                                let company_name: String = job.get("company_name");
                                let salary_range: String = job.get("salary_range");
                                let location: String = job.get("location");
                                let title: String = job.get("job_title");
                                let formatted_string = format!("Company name: {company_name}\nTitle: {title}\nSalary Range: {salary_range}\nLocation: {location}\n\n");
                                job_string.push_str(&formatted_string);
                            }
                            bot.send_message(msg.chat.id, format!("{job_string}\n"))
                                .await?;
                        }
                        Err(_) => {
                            bot.send_message(
                                msg.chat.id,
                                format!("\nYou haven't posted any jobs. To start type /post"),
                            )
                            .await?;
                        }
                    }
                }
                Err(_) => {
                    bot.send_message(
                        msg.chat.id,
                        format!("\nYou haven't posted any jobs. To start type /post"),
                    )
                    .await?;
                }
            }
        }
    }
    Ok(())
}

async fn handle_other_updates(bot: Bot, msg: Message) -> BotResult {
    bot.send_message(msg.chat.id, "To see the list of commands type /help")
        .await?;
    Ok(())
}

pub async fn run(state: SharedState) {
    let bot = Bot::new(&state.teloxide_token);

    Dispatcher::builder(
        bot,
        dptree::entry()
            .branch(
                Update::filter_message()
                    .enter_dialogue::<Message, InMemStorage<State>, State>()
                    .filter_command::<Command>()
                    .endpoint(answer),
            )
            .branch(
                Update::filter_message()
                    .enter_dialogue::<Message, InMemStorage<State>, State>()
                    .branch(dptree::case![State::ReceiveCompanyName].endpoint(receive_company_name))
                    .branch(
                        dptree::case![State::ReceiveTitle { company_name }]
                            .endpoint(receive_job_title),
                    )
                    .branch(
                        dptree::case![State::ReceiveSalaryRange {
                            company_name,
                            title
                        }]
                        .endpoint(receive_salary_range),
                    )
                    .branch(
                        dptree::case![State::ReceiveLocation {
                            company_name,
                            title,
                            salary_range
                        }]
                        .endpoint(receive_location),
                    ),
            )
            .branch(Update::filter_message().endpoint(handle_other_updates)),
    )
    .dependencies(dptree::deps![state, InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}
