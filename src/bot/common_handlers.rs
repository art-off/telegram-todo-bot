use std::error::Error;
use std::fmt::format;
use std::sync::Arc;
use teloxide::{prelude::*, Bot};
use crate::bot::keyboard::make_update_todos_status_keyboard;
use crate::BotState;
use crate::constants::Constants;
use crate::database::models::{TodoItem, TodoItemStatus, TodoList};
use crate::database::repository::{TodoItemRepository};
use crate::utils::number_parser::{NumberParser, NumberParserMode};

pub async fn handle_list_command(
    bot: Bot,
    msg: Message,
    state: Arc<BotState>
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let user = msg.from().unwrap();
    let todo_item_repository = TodoItemRepository::new(state.connection.clone());

    let user_todo_list = todo_item_repository.get_todos(user);
    send_todo_list(bot, user_todo_list, msg.chat.id).await?;

    Ok(())
}

pub async fn handle_done_command(
    bot: Bot,
    msg: Message,
    state: Arc<BotState>,
    todo_item_num: usize
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let user = msg.from().unwrap();
    let todo_item_repository = TodoItemRepository::new(state.connection.clone());
    let todo_list = todo_item_repository.get_todos(user);
    let todo_item = todo_list.todo_items.get(todo_item_num);
    match todo_item {
        Some(todo_item) => {
            todo_item_repository.update_status(todo_item, TodoItemStatus::Done);
            let updated_todo_list = todo_item_repository.get_todos(user);
            send_todo_list(bot, updated_todo_list, msg.chat.id).await?;
        },
        None => {
            bot.send_message(
                msg.chat.id,
                format!("Todo with index {} not found", todo_item_num)
            ).await?;
        },
    }

    Ok(())
}

pub async fn handle_delete_command(
    bot: Bot,
    msg: Message,
    state: Arc<BotState>,
    todo_item_nums_to_delete: String
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let user = msg.from().unwrap();
    let todo_item_repository = TodoItemRepository::new(state.connection.clone());

    let number_parser = NumberParser::new(
        todo_item_nums_to_delete,
        vec![NumberParserMode::SingleNumbers, NumberParserMode::NumberRanges],
    );

    let todo_item_nums = number_parser.parse();
    let todo_list = todo_item_repository.get_todos(user);

    let todo_items_to_delete: Vec<&TodoItem> = todo_item_nums.iter()
        .map(|num| todo_list.todo_items.get(*num as usize))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    for todo_item in todo_items_to_delete {
        todo_item_repository.delete_todo(todo_item.id)
    }

    let updated_todo_list = todo_item_repository.get_todos(user);
    send_todo_list(bot, updated_todo_list, msg.chat.id).await?;

    Ok(())
}


async fn send_todo_list(
    bot: Bot, todo_list:
    TodoList,
    chat_id: ChatId
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let keyboard = make_update_todos_status_keyboard(&todo_list);

    bot.send_message(
        chat_id,
        Constants::list_message_string()
    )
        .reply_markup(keyboard)
        .await?;

    Ok(())
}