use teloxide::types::User;
use diesel::prelude::Queryable;
use diesel::sql_types::Integer;

#[repr(u8)]
pub enum TodoItemStatus {
    New = 0,
    Done = 1,
}

#[derive(Queryable)]
pub struct TodoItem {
    pub id: i32,
    pub text: String,
    pub status: i16,
    pub tg_user_id: i32,
}

pub struct TodoList {
    pub(crate) todo_items: Vec<TodoItem>
}

impl TodoItemStatus {
    // TODO наверное есть встроенный красивый механизм
    pub fn tg_display(&self) -> String {
        match self {
            TodoItemStatus::New => String::from("New"),
            TodoItemStatus::Done => String::from("Done"),
        }
    }
}

impl TodoItem {
    fn tg_display(&self) -> String {
        format!("[{}] {}", self.status_as_enum().tg_display(), self.text)
    }
}

impl TodoItem {
    fn status_as_enum(&self) -> TodoItemStatus {
        TodoItemStatus::try_from(self.status).unwrap_or(TodoItemStatus::New)
    }
}

impl TodoList {
    pub fn tg_display(&self) -> String {
        self.todo_items.iter()
            .map(|x| format!("- {}", x.tg_display()))
            .collect::<Vec<String>>()
            .join("\n")
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