use crate::diesel_migrations::MigrationHarness;
use anyhow::{Context, Result};
use diesel::{Connection, SqliteConnection};
use diesel_migrations::EmbeddedMigrations;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn database_exists() -> Result<String> {
	let database_path = dirs::data_dir()
		.context("Failed to find data directory")?
		.join("done/dev.edfloreshz.Done.db");
	if !database_path.exists() {
		std::fs::File::create(&database_path)?;
	}
	let path = database_path.to_str().context("Failed to convert path to string")?;
	Ok(path.to_string())
}

pub fn establish_connection() -> Result<SqliteConnection> {
	let url = database_exists()?;

	let mut connection = SqliteConnection::establish(url.as_str()).context("Error connecting to database")?;
	connection.run_pending_migrations(MIGRATIONS).unwrap();
	Ok(connection)
}
