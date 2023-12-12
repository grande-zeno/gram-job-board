pub mod posts;
pub mod edits;
pub mod delete;

use crate::SharedState;
use posts::*;
use std::error::Error;
use teloxide::{
    dispatching::dialogue::InMemStorage, 
    utils::command::BotCommands,
    prelude::*
};

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
    #[command(description = "handles editing jobs")]
    Edit,
    #[command(description = "handles deleting jobs. Use the code name to delete a job")]
    Delete(String),
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
        },
        Command::Post => {
            bot.send_message(msg.chat.id, "Let's start! What's your company name?")
                .await?;
            dialogue.update(State::ReceiveCompanyName).await?;
        },
        Command::MyJobs => posts::myjobs(bot, state, msg).await?,
        Command::Edit => {
            bot.send_message(msg.chat.id, "Let's edit this")
                .await?;
        },
        Command::Delete(job) => delete::delete_job(job, bot, msg, state).await?
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
            .branch(posts::post_updates())
            .branch(Update::filter_message().endpoint(handle_other_updates)),
    )
    .dependencies(dptree::deps![state, InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}
