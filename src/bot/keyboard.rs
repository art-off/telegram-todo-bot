use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use crate::database::models::TodoList;

pub fn make_update_todos_status_keyboard(todo_list: &TodoList) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let buttons: Vec<InlineKeyboardButton> = todo_list.todo_items.iter()
        .enumerate()
        .map(|x|
            InlineKeyboardButton::callback(x.0.to_string(), x.0.to_string())
        )
        .collect();

    for nums_row in buttons.chunks(5) {
        keyboard.push(nums_row.to_vec());
    }

    InlineKeyboardMarkup::new(keyboard)
}
