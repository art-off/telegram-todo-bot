use diesel::prelude::*;

pub struct TodoList {
    pub todo_items: Vec<TodoItem>
}

#[derive(Queryable)]
pub struct TodoItem {
    pub id: i32,
    pub text: String,
    pub status: i16,
    pub tg_user_id: i32,
}

#[repr(u8)]
pub enum TodoItemStatus {
    New = 0,
    Done = 1,
}

impl TodoList {
    pub fn new(todo_items: Vec<TodoItem>) -> Self {
        Self { todo_items }
    }
}

impl TryFrom<i16> for TodoItemStatus {
    type Error = ();

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::New,
            1 => Self::Done,
            _ => return Err(()),
        })
    }
}

impl TodoItem {
    pub fn status_as_enum(&self) -> TodoItemStatus {
        TodoItemStatus::try_from(self.status).unwrap_or(TodoItemStatus::New)
    }
}