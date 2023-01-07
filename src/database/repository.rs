use std::sync::{Arc, Mutex};
use diesel::{prelude::*, SqliteConnection};
use teloxide::types::{Message, User};

use crate::database::models::{LastListMessage, TodoItem, TodoList};

use crate::database::schema::todos as TodoItemSchema;
use crate::database::schema::last_list_message as LastListMessageSchema;

pub struct TodoItemRepository {
    connection: Arc<Mutex<SqliteConnection>>
}

impl TodoItemRepository {
    pub fn new(connection: Arc<Mutex<SqliteConnection>>) -> Self {
        Self { connection }
    }

    pub fn get_new_todos(&self, user: &User) -> TodoList {
        let user_id = user.id.0 as i32;
        let result = TodoItemSchema::table
            .filter(TodoItemSchema::dsl::tg_user_id.eq(user_id))
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

pub struct LastListMessageRepository {
    connection: Arc<Mutex<SqliteConnection>>
}

impl LastListMessageRepository {
    pub fn new(connection: Arc<Mutex<SqliteConnection>>) -> Self {
        Self { connection }
    }

    pub fn get_last_list_message(&self, user: &User) -> Option<LastListMessage> {
        let user_id = user.id.0 as i32;

        let result: Option<LastListMessage> = LastListMessageSchema::table
            .filter(LastListMessageSchema::tg_user_id.eq(user_id))
            .first(&mut *self.connection.lock().unwrap())
            .optional()
            .expect("Error loading LastListMessage");;

        result
    }

    // TODO вот сюда бы key-value хранилище
    pub fn save_last_list_message(&self, msg: Message, user: &User) {
        let user_id = user.id.0 as i32;
        let msg_id = msg.id.0;

        diesel::delete(
            LastListMessageSchema::table
                .filter(LastListMessageSchema::dsl::tg_user_id.eq(user_id))
        )
            .execute(&mut *self.connection.lock().unwrap())
            .expect("LastListMessage deleting is falling");

        diesel::insert_into(LastListMessageSchema::table)
            .values(
                (
                    LastListMessageSchema::dsl::message_id.eq(msg_id),
                    LastListMessageSchema::dsl::tg_user_id.eq(user_id),
                )
            )
            .execute(&mut *self.connection.lock().unwrap())
            .expect("Error saving new LastListMessage");
    }
}