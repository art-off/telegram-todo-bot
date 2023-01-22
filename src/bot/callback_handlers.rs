use std::error::Error;
use std::sync::Arc;
use teloxide::Bot;
use teloxide::types::{CallbackQuery};
use teloxide::prelude::*;
use crate::bot::keyboard::make_update_todos_status_keyboard;
use crate::BotState;
use crate::database::repository::{TodoItemRepository};
use crate::presenting::todo_item::tg_display_todo_list;

pub async fn handle_callback(
    bot: Bot,
    q: CallbackQuery,
    state: Arc<BotState>
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Вдруг еще появятся разные КАЛбеки
    handle_update_todo_status_callback(bot, q, state).await
}

async fn handle_update_todo_status_callback(
    bot: Bot,
    q: CallbackQuery,
    state: Arc<BotState>
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let todo_item_repository = TodoItemRepository::new(state.connection.clone());
    let user = &q.from;

    if let Some(string_todo_id) = q.data {
        bot.answer_callback_query(q.id).await?;

        let todo_item_id: i32 = string_todo_id.parse().unwrap();
        let selected_todo = &todo_item_repository.get_todo(todo_item_id);

        if let (Some(selected_todo), Some(mes)) = (selected_todo, q.message) {
            todo_item_repository.update_status(
                selected_todo,
                selected_todo.status_as_enum().toggled()
            );

            let updated_todo_list = todo_item_repository.get_todos(user);
            let keyboard = make_update_todos_status_keyboard(&updated_todo_list);
            bot.edit_message_text(
                mes.chat.id,
                mes.id,
                tg_display_todo_list(&updated_todo_list)
            )
                .reply_markup(keyboard)
                .await?;
        }
    }

    Ok(())
}