use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub struct DatabaseConnection;

impl DatabaseConnection {
    pub fn establish_connection() -> SqliteConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| dirs::data_dir().unwrap().join("do/org.devloop.Do.db").display().to_string());

        SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}
