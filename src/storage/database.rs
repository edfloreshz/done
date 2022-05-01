use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub struct DatabaseConnection;

impl DatabaseConnection {
    pub fn establish_connection() -> SqliteConnection {
        dotenv().ok();
        let database_url = match env::var("DATABASE_URL") {
            Ok(url) => url,
            Err(_) => format!("{}/.local/share/do/com.devloop.do.db", dirs::home_dir().unwrap().display())
        };
        SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}
