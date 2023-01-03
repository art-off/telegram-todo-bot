extern crate core;

mod todo_item;
mod tg_user_command;
mod schema;

use std::env;
use diesel::{Connection, QueryDsl, RunQueryDsl, SqliteConnection};
use dotenvy::dotenv;
use tg_user_command::TgUserCommand;
use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::types::{User, Message, MessageKind};
use crate::todo_item::TodoItem;

use self::schema::todos;

#[tokio::main]
async fn main() {
    use self::schema::todos::dsl::*;

    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let connection = &mut establish_connection();
    let results = todos
        .load::<TodoItem>(connection)
        .expect("Error loading todos");

    println!("Displaying {} todos", results.len());
    for todo in results {
        println!("{} {} {}", todo.id, todo.text, todo.status);
    }

    let bot = Bot::from_env();
    TgUserCommand::repl(bot, answer).await;
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
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