use std::{error::Error, sync::Arc};
use teloxide::{prelude::*, Bot, types::Message, utils::command::BotCommands};
use crate::BotState;
use crate::database::repository::TodoItemRepository;
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
}

pub async fn handle(bot: Bot, msg: Message, cmd: Command, state: Arc<BotState>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let todo_item_repository = TodoItemRepository::new(state.connection.clone());
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
            let user_todo_list = todo_item_repository.get_new_todos(user);
            let message = tg_display_todo_list(user_todo_list);
            bot.send_message(msg.chat.id, message).await?;
        },
    };

    Ok(())
}