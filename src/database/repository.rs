use std::sync::{Arc, Mutex};
use diesel::{prelude::*, SqliteConnection};
use teloxide::types::{User};
use diesel::OptionalExtension;

use crate::database::models::{TodoItem, TodoItemStatus, TodoList};

use crate::database::schema::todos as TodoItemSchema;

pub struct TodoItemRepository {
    connection: Arc<Mutex<SqliteConnection>>
}

impl TodoItemRepository {
    pub fn new(connection: Arc<Mutex<SqliteConnection>>) -> Self {
        Self { connection }
    }

    pub fn get_todos(&self, user: &User) -> TodoList {
        let user_id = user.id.0 as i32;
        let result = TodoItemSchema::table
            .filter(TodoItemSchema::dsl::tg_user_id.eq(user_id))
            .load::<TodoItem>(&mut *self.connection.lock().unwrap())
            .expect("Error loading todos");

        TodoList::new(result)
    }

    pub fn get_todo(&self, todo_item_id: i32) -> Option<TodoItem> {
        TodoItemSchema::table
            .filter(TodoItemSchema::dsl::id.eq(todo_item_id))
            .first(&mut *self.connection.lock().unwrap())
            .optional()
            .unwrap()
    }

    pub fn delete_todo(&self, todo_item_id: i32) {
        diesel::delete(
            TodoItemSchema::table.filter(
                TodoItemSchema::dsl::id.eq(todo_item_id)
            )
        )
            .execute(&mut *self.connection.lock().unwrap())
            .expect("Error deleting todo");
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

    pub fn update_status(&self, todo_item: &TodoItem, status: TodoItemStatus) {
        diesel::update(
            TodoItemSchema::table
                .filter(TodoItemSchema::dsl::id.eq(todo_item.id))
        )
            .set(TodoItemSchema::dsl::status.eq(status as i16))
            .execute(&mut *self.connection.lock().unwrap())
            .expect("Error updating status");
    }
}