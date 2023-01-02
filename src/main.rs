// mod command;
mod todo_item;
mod tg_user_command;

use tg_user_command::TgUserCommand;
use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::types::{User, Message, MessageKind};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();
    TgUserCommand::repl(bot, answer).await;
}

async fn answer(bot: Bot, msg: Message, cmd: TgUserCommand) -> ResponseResult<()> {
    match cmd {
        TgUserCommand::Help => bot.send_message(msg.chat.id, TgUserCommand::descriptions().to_string()).await?,
        TgUserCommand::New { todo_text } => {
            match user_by_msg(msg.clone()) {
                Some(user) => bot.send_message(msg.chat.id, user.id.to_string()).await?,
                None => bot.send_message(msg.chat.id, "No user").await?, // TODO Удалить это
            }
        },
        TgUserCommand::List => {
            match user_by_msg(msg.clone()) {
                Some(user) => {
                    let user_todo_list = todo_item::todo_list_for_user(user).await;
                    bot.send_message(msg.chat.id, user_todo_list.expect("kek error :(").tg_display()).await?
                }
                None => bot.send_message(msg.chat.id, "No user").await?, // TODO Удалить это
            }
        },
    };

    Ok(())
}

fn user_by_msg(msg: Message) -> Option<User> {
    match msg.kind {
        MessageKind::Common(message_common) => message_common.from,
        _ => None
    }
}