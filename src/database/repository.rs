use std::sync::{Arc, Mutex};
use diesel::{prelude::*, SqliteConnection};
use teloxide::types::User;

use crate::database::models::{TodoItem, TodoList};
use crate::database::schema::todos as TodoItemSchema;

pub struct TodoItemRepository {
    connection: Arc<Mutex<SqliteConnection>>
}

impl TodoItemRepository {
    pub fn new(connection: Arc<Mutex<SqliteConnection>>) -> Self {
        Self { connection }
    }

    pub fn get_new_todos(&self, user: &User) -> TodoList {
        let result = TodoItemSchema::table
            .filter(TodoItemSchema::dsl::tg_user_id.eq(user.id.0 as i32))
            .filter(TodoItemSchema::dsl::status.eq(0))
            .load::<TodoItem>(&mut *self.connection.lock().unwrap())
            .expect("Error loading todos");

        TodoList::new(result)
    }

    pub fn add_todo(&self, todo_text: &str, user: &User) {
        diesel::insert_into(TodoItemSchema::table)
            .values(
                (
                    TodoItemSchema::dsl::text.eq(todo_text),
                    TodoItemSchema::dsl::status.eq(0),
                    TodoItemSchema::dsl::tg_user_id.eq(user.id.0 as i32),
                )
            )
            .execute(&mut *self.connection.lock().unwrap())
            .expect("Error saving new todo");
    }
}