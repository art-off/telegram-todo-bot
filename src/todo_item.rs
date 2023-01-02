use teloxide::types::User;

pub enum TodoItemStatus {
    New,
    Done,
}

impl TodoItemStatus {
    // TODO наверное есть встроенный красивый механизм
    pub fn td_display(&self) -> String {
        match self {
            TodoItemStatus::New => String::from("New"),
            TodoItemStatus::Done => String::from("Done"),
        }
    }
}

pub struct TodoItem {
    status: TodoItemStatus,
    text: String,
}

impl TodoItem {
    fn tg_display(&self) -> String {
        format!("[{}] {}", self.status.td_display(), self.text)
    }
}

pub struct TodoList {
    todo_items: Vec<TodoItem>
}

impl TodoList {
    pub fn tg_display(&self) -> String {
        self.todo_items.iter()
            .map(|x| format!("- {}", x.tg_display()))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

pub async fn todo_list_for_user(_user: User) -> Option<TodoList>{
    Some(TodoList { todo_items: vec![
        TodoItem { status: TodoItemStatus::Done, text: String::from("First") },
        TodoItem { status: TodoItemStatus::New, text: String::from("Second") },
        TodoItem { status: TodoItemStatus::Done, text: String::from("Third") },
    ]})
}

pub async fn add_new_todo_item_for_user(user: User) {

}