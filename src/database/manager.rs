use std::{
    env,
    sync::{Arc, Mutex},
};
use diesel::{
    prelude::*,
    SqliteConnection
};

pub struct DBManager {
    pub database_env_key: String,
}

impl DBManager {
    pub fn new(database_env_key: String) -> Self {
        Self { database_env_key }
    }

    pub fn establish_connection(&self) -> Arc<Mutex<SqliteConnection>> {
        let database_url = env::var(&self.database_env_key)
            .expect(&format!("{} must be set", self.database_env_key));

        Arc::new(Mutex::new(
            SqliteConnection::establish(&database_url)
                .unwrap_or_else(|_| panic!("Error connecting to {}", database_url)),
        ))
    }
}