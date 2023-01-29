use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use crate::database::models::TodoList;
use crate::presenting::todo_item::tg_display_todo_item;

pub fn make_update_todos_status_keyboard(todo_list: &TodoList) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let buttons: Vec<InlineKeyboardButton> = todo_list.todo_items.iter()
        .enumerate()
        .map(|x|
            InlineKeyboardButton::callback(
                tg_display_todo_item(x.0, x.1),
                x.1.id.to_string(),
            )
        )
        .collect();

    for nums_row in buttons {
        keyboard.push(vec![nums_row]);
    }

    InlineKeyboardMarkup::new(keyboard)
}