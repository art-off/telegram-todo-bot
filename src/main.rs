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
use crate::todo_item::{TodoItem, TodoList};

use std::sync::{Arc, Mutex};

use self::schema::todos;
use self::schema::todos::dsl::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let connection = establish_connection();
    let bot = Bot::from_env();

    TgUserCommand::repl(
        bot,
        move |bot, msg, cmd|
            answer(bot, msg, cmd, connection.clone())
    ).await;
}

fn establish_connection() -> Arc<Mutex<SqliteConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    Arc::new(Mutex::new(
        SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url)),
    ))
}

async fn answer(bot: Bot, msg: Message, cmd: TgUserCommand, connection: Arc<Mutex<SqliteConnection>>) -> ResponseResult<()> {
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
                    let results = todos
                        .load::<TodoItem>(&mut *connection.lock().unwrap())
                        .expect("Error loading todos");
                    bot.send_message(msg.chat.id, TodoList { todo_items: results }.tg_display()).await?
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