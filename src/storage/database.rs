use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub struct DatabaseConnection;

impl DatabaseConnection {
    pub fn establish_connection() -> SqliteConnection {
        let database_path = dirs::data_dir().unwrap().join("do/org.devloop.Do.db");
        let database_url = database_path.to_str().unwrap();
        SqliteConnection::establish(database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}
