use std::error::Error;
use std::os::macos::raw::stat;
use std::sync::Arc;
use teloxide::{prelude::*, Bot, utils::command::BotCommands};
use teloxide::types::{MessageId, User};
use crate::BotState;
use crate::database::models::{TodoItemStatus, TodoList};
use crate::database::repository::{LastListMessageRepository, TodoItemRepository};
use crate::presenting::todo_item::tg_display_todo_list;

pub async fn handle_list_command(bot: Bot, msg: Message, state: Arc<BotState>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let user = msg.from().unwrap();
    let todo_item_repository = TodoItemRepository::new(state.connection.clone());
    let last_list_message_repository = LastListMessageRepository::new(state.connection.clone());

    let user_todo_list = todo_item_repository.get_todos(user);
    send_todo_list(bot, user_todo_list, msg.chat.id, user, state).await?;

    Ok(())
}

pub async fn handle_done_command(bot: Bot, msg: Message, state: Arc<BotState>, todo_item_num: usize) -> Result<(), Box<dyn Error + Send + Sync>> {
    let user = msg.from().unwrap();
    let todo_item_repository = TodoItemRepository::new(state.connection.clone());
    let last_list_message_repository = LastListMessageRepository::new(state.connection.clone());

    let todo_list = todo_item_repository.get_todos(user);
    let todo_item = todo_list.todo_items.get(todo_item_num);
    match todo_item {
        Some(todo_item) => {
            todo_item_repository.update_status(todo_item, TodoItemStatus::Done, user);
            let updated_todo_list = todo_item_repository.get_todos(user);
            if let Some(_last_list_message) = last_list_message_repository.get_last_list_message(user) {
                send_todo_list(bot, updated_todo_list, msg.chat.id, user, state).await?;
            }
        },
        None => {
            bot.send_message(msg.chat.id, format!("Todo with index {} not found", todo_item_num)).await?;
        },
    }

    Ok(())
}

pub async fn handle_delete_command(bot: Bot, msg: Message, state: Arc<BotState>, todo_item_num: usize) -> Result<(), Box<dyn Error + Send + Sync>> {
    let user = msg.from().unwrap();
    let todo_item_repository = TodoItemRepository::new(state.connection.clone());

    let todo_list = todo_item_repository.get_todos(user);
    let todo_item = todo_list.todo_items.get(todo_item_num);
    match todo_item {
        Some(todo_item) => {
            todo_item_repository.delete_todo(todo_item.id);
            let updated_todo_list = todo_item_repository.get_todos(user);
            send_todo_list(bot, updated_todo_list, msg.chat.id, user, state).await?;
        },
        None => {
            bot.send_message(msg.chat.id, format!("Todo with index {} not found", todo_item_num)).await?;
        },
    }

    Ok(())
}


async fn send_todo_list(bot: Bot, todo_list: TodoList, chat_id: ChatId, user: &User, state: Arc<BotState>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let last_list_message_repository = LastListMessageRepository::new(state.connection.clone());

    let message = tg_display_todo_list(todo_list);
    let bot_msg = bot.send_message(chat_id, message).await?;
    last_list_message_repository.save_last_list_message(bot_msg, user);

    Ok(())
}