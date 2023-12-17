pub mod checkpoint;
pub mod cmd_checkpoint;
pub mod codename;
pub mod company;
pub mod location;
pub mod salary;
pub mod start;
pub mod title;

use super::BotResult;
pub use start::start;
use teloxide::{
    dispatching::{dialogue::InMemStorage, DpHandlerDescription},
    prelude::*,
};

pub type EditDialogue = Dialogue<EditState, InMemStorage<EditState>>;

#[derive(Clone, Default)]
pub enum EditState {
    #[default]
    Start,
    ReceiveCodeName,
    CheckPoint {
        codename: String,
    },
    CmdCheckPoint {
        codename: String,
        cmd: String,
    }
}

pub fn edit_updates() -> Handler<'static, DependencyMap, BotResult, DpHandlerDescription> {
    Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<EditState>, EditState>()
        .branch(dptree::case![EditState::ReceiveCodeName].endpoint(codename::receive_codename))
        .branch(dptree::case![EditState::CheckPoint { codename }].endpoint(checkpoint::checkpoint))
        .branch(
            dptree::case![EditState::CmdCheckPoint { codename, cmd }]
                .endpoint(cmd_checkpoint::cmd_checkpoint),
        )
}
