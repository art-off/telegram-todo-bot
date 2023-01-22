use crate::database::models::{TodoItem, TodoItemStatus, TodoList};

pub fn tg_display_todo_list(todo_list: &TodoList) -> String {
    todo_list.todo_items.iter()
        .enumerate()
        .map(|x| format!("{}: {}", x.0, tg_display_todo_item(x.1)))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn tg_display_todo_item(todo_item: &TodoItem) -> String {
    format!("[{}] {}", tg_display_status(todo_item.status_as_enum()), todo_item.text)
}

pub fn tg_display_status(status: TodoItemStatus) -> String {
    // TODO наверное есть встроенный красивый механизм
    match status {
        TodoItemStatus::New => String::from("New"),
        TodoItemStatus::Done => String::from("Done"),
    }
}