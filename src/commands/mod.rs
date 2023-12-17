pub mod delete;
pub mod edits;
pub mod myjobs;
pub mod posts;

use crate::SharedState;
pub use edits::*;
pub use posts::*;
use std::error::Error;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*, utils::command::BotCommands};

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
    edit_dialogue: EditDialogue,
    state: SharedState,
) -> BotResult {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Post => posts::start(bot, dialogue, msg).await?,
        Command::MyJobs => myjobs::myjobs(bot, state, msg).await?,
        Command::Edit => edits::start(bot, edit_dialogue, msg).await?,
        Command::Delete(code_name) => delete::delete_job(code_name, bot, msg, state).await?,
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
                    .enter_dialogue::<Message, InMemStorage<EditState>, EditState>()
                    .filter_command::<Command>()
                    .endpoint(answer),
            )
            .branch(posts::post_updates())
            .branch(edits::edit_updates())
            .branch(Update::filter_message().endpoint(handle_other_updates)),
    )
    .dependencies(dptree::deps![
        state,
        InMemStorage::<State>::new(),
        InMemStorage::<EditState>::new()
    ])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}
