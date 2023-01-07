use std::{error::Error, sync::Arc};
use teloxide::{prelude::*, Bot, types::Message, utils::command::BotCommands};
use crate::bot::common_handlers::{handle_done_command, handle_list_command};
use crate::BotState;
use crate::database::repository::{LastListMessageRepository, TodoItemRepository};
use crate::presenting::todo_item::tg_display_todo_list;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Display this text")]
    Help,
    #[command(description = "Create new todo")]
    New { todo_text: String, },
    #[command(description = "Show list of new todos")]
    List,
    #[command(description = "Set status to `Done`")]
    Done { todo_item_num: usize },
    #[command(description = "Delete todo")]
    Delete,
    #[command(description = "Kek i kek")]
    Kek,
}

pub async fn handle(bot: Bot, msg: Message, cmd: Command, state: Arc<BotState>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let todo_item_repository = TodoItemRepository::new(state.connection.clone());
    let last_list_message_repository = LastListMessageRepository::new(state.connection.clone());
    let user = msg.from().unwrap();

    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        },
        Command::New { todo_text } => {
            todo_item_repository.add_todo(&todo_text, user);
            bot.send_message(msg.chat.id, "Ok").await?;
        },
        Command::List => {
            handle_list_command(bot, msg, state.clone()).await?;
        },
        Command::Done { todo_item_num } => {
            handle_done_command(bot, msg, state.clone(), todo_item_num).await?;
        },
        Command::Delete => {

        },
        Command::Kek => {
            let last_list_message = last_list_message_repository.get_last_list_message(user);

            if let Some(last_list_message) = last_list_message {
                bot.send_message(msg.chat.id, last_list_message.message_id.to_string()).await?;
            }
        },
    };

    Ok(())
}