use diesel_migrations::EmbeddedMigrations;
use crate::diesel_migrations::MigrationHarness;
use anyhow::{Context, Result};
use diesel::{Connection, SqliteConnection};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn _establish_connection() -> Result<SqliteConnection> {
	let database_path = dirs::data_dir()
		.with_context(|| "Failed to get plugins directory.")?
		.join("done/dev.edfloreshz.Done.db");
	let database_url = database_path
		.to_str()
		.with_context(|| "Failed to convert path to string")?;
	let mut connection = SqliteConnection::establish(database_url)
		.with_context(|| "Error connecting to database")?;
	connection.run_pending_migrations(MIGRATIONS).unwrap();

	Ok(connection)
}
