extern crate core;

mod utils;
mod database;
mod bot;
mod presenting;
mod constants;

use std::sync::{Arc, Mutex};
use dotenvy::dotenv;
use diesel::SqliteConnection;
use teloxide::prelude::*;

use bot::commands as BotCommands;
use crate::bot::callback_handlers::handle_callback;
use crate::database::manager::DBManager;

pub struct BotState {
    pub connection: Arc<Mutex<SqliteConnection>>,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    dotenv().ok();

    let db_manager = DBManager::new(String::from("DATABASE_URL"));
    let connection = db_manager.establish_connection();

    let bot_state = Arc::new(
        BotState { connection }
    );

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<BotCommands::Command>()
                .endpoint(BotCommands::handle)
        )
        .branch(
            Update::filter_callback_query()
                .endpoint(handle_callback)
        );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![Arc::clone(&bot_state)])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await
}