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
        format!("[{}] {}", 4, self.text)
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

impl TryFrom<u8> for TodoItemStatus {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::New,
            1 => Self::Done,
            _ => return Err(()),
        })
    }
}

pub async fn todo_list_for_user(_user: User) -> Option<TodoList>{
    Some(TodoList { todo_items: vec![] })
}

pub async fn add_new_todo_item_for_user(_user: User) {

}