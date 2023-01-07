use std::error::Error;
use std::sync::Arc;
use teloxide::{prelude::*, Bot, utils::command::BotCommands};
use teloxide::types::User;
use crate::BotState;
use crate::database::repository::{LastListMessageRepository, TodoItemRepository};
use crate::presenting::todo_item::tg_display_todo_list;

pub async fn handle_list_command(bot: Bot, msg: Message, state: Arc<BotState>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let user = msg.from().unwrap();
    let todo_item_repository = TodoItemRepository::new(state.connection.clone());
    let last_list_message_repository = LastListMessageRepository::new(state.connection.clone());

    let user_todo_list = todo_item_repository.get_new_todos(user);
    let message = tg_display_todo_list(user_todo_list);
    let bot_msg = bot.send_message(msg.chat.id, message).await?;
    last_list_message_repository.save_last_list_message(bot_msg, user);

    Ok(())
}
