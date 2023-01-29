use crate::database::models::{TodoItem, TodoItemStatus, TodoList};

#[warn(dead_code)]
pub fn tg_display_todo_list(todo_list: &TodoList) -> String {
    todo_list.todo_items.iter()
        .enumerate()
        .map(|x| format!("{}", tg_display_todo_item(x.0, x.1)))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn tg_display_todo_item(index: usize, todo_item: &TodoItem) -> String {
    format!("{}: {} {}", index, tg_display_status(todo_item.status_as_enum()), todo_item.text)
}

pub fn tg_display_status(status: TodoItemStatus) -> String {
    match status {
        TodoItemStatus::New => String::from("🔴"),
        TodoItemStatus::Done => String::from("✅"),
    }
}