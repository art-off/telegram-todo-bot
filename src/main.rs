extern crate core;

mod models;
mod command;
mod schema;

use std::{
    env,
    sync::{Arc, Mutex}
};
use diesel::{
    prelude::*,
    SqliteConnection,
};
use teloxide::{
    prelude::*,
    utils::command::BotCommands,
    types::{User, Message}
};
use dotenvy::dotenv;

use command::Command;
use crate::schema::todos as TodoItemSchema;
use crate::models::{TodoItem, TodoList};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let connection = establish_connection();
    let bot = Bot::from_env();

    Command::repl(
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

async fn answer(bot: Bot, msg: Message, cmd: Command, connection: Arc<Mutex<SqliteConnection>>) -> ResponseResult<()> {
    let user = msg.from().unwrap();
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::New { todo_text } => {
            add_new_todo_for_user(user, &todo_text, connection);
            bot.send_message(msg.chat.id, "Ok").await?
        },
        Command::List => {
            let user_todo_list = todo_list_for_user(user, connection);
            bot.send_message(msg.chat.id, user_todo_list.tg_display()).await?
        },
    };

    Ok(())
}

fn add_new_todo_for_user(user: &User, todo_text: &str, connection: Arc<Mutex<SqliteConnection>>) {
    diesel::insert_into(TodoItemSchema::table)
        .values(
            (
                TodoItemSchema::dsl::text.eq(todo_text),
                TodoItemSchema::dsl::status.eq(0),
                TodoItemSchema::dsl::tg_user_id.eq(user.id.0 as i32),
            )
        )
        .execute(&mut *connection.lock().unwrap())
        .expect("Error saving new todo");
}

fn todo_list_for_user(user: &User, connection: Arc<Mutex<SqliteConnection>>) -> TodoList {
    let results = TodoItemSchema::table
        .filter(TodoItemSchema::dsl::tg_user_id.eq(user.id.0 as i32))
        .load::<TodoItem>(&mut *connection.lock().unwrap())
        .expect("Error loading todos");

    TodoList::new(results)
}