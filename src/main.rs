extern crate core;

mod models;
mod tg_user_command;
mod schema;

use std::env;
use diesel::{Connection, QueryDsl, RunQueryDsl, SqliteConnection};
use diesel::prelude::*;
use dotenvy::dotenv;
use tg_user_command::TgUserCommand;
use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::types::{User, Message, MessageKind};
use crate::models::{TodoItem, TodoItemStatus, TodoList};

use std::sync::{Arc, Mutex};
use diesel::result::Error;
use diesel::{dsl, prelude::*};

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
            match msg.from() {
                Some(user) => {
                    add_new_todo_for_user(user, &todo_text, connection);
                    bot.send_message(msg.chat.id, user.id.to_string()).await?
                }
                None => bot.send_message(msg.chat.id, "No user").await?, // TODO Удалить это
            }
        },
        TgUserCommand::List => {
            match msg.from() {
                Some(user) => {
                    let user_todo_list = todo_list_for_user(user, connection);
                    bot.send_message(msg.chat.id, user_todo_list.tg_display()).await?
                }
                None => bot.send_message(msg.chat.id, "No user").await?, // TODO Удалить это
            }
        },
    };

    Ok(())
}

fn add_new_todo_for_user(user: &User, todo_text: &str, connection: Arc<Mutex<SqliteConnection>>) {
    diesel::insert_into(todos::table)
        .values(
            (
                todos::dsl::text.eq(todo_text),
                todos::dsl::status.eq(0),
                todos::dsl::tg_user_id.eq(user.id.0 as i32),
            )
        )
        .execute(&mut *connection.lock().unwrap())
        .expect("TODO: panic message");
}

fn todo_list_for_user(user: &User, connection: Arc<Mutex<SqliteConnection>>) -> TodoList {
    let results = todos
        .filter(tg_user_id.eq(user.id.0 as i32))
        .load::<TodoItem>(&mut *connection.lock().unwrap())
        .expect("Error loading todos");

    TodoList::new(results)
}