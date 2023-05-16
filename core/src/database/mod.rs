mod models;
pub use models::*;

use anyhow::{anyhow, Context, Result};
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{
	embed_migrations, EmbeddedMigrations, MigrationHarness,
};
use libset::project::Project;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
pub const DATABASE_NAME: &str = "dev.edfloreshz.Done.db";

pub struct Database;

impl Database {
	fn database_url() -> Result<String> {
		let url = Project::open("dev", "edfloreshz", "done")?
			.path()
			.context("The project has not been created")?
			.join(DATABASE_NAME)
			.to_str()
			.context("Failed to convert path to string")?
			.to_string();
		Ok(url)
	}

	pub fn establish_connection() -> Result<SqliteConnection> {
		SqliteConnection::establish(Database::database_url()?.as_str())
			.context("Error connecting to database")
	}

	pub fn ensure_migrations_up_to_date() -> Result<()> {
		let mut connection =
			SqliteConnection::establish(Database::database_url()?.as_str())
				.context("Error connecting to database")?;
		match connection.run_pending_migrations(MIGRATIONS) {
			Ok(_) => Ok(()),
			Err(err) => {
				tracing::error!("{err}");
				Err(anyhow!(err))
			},
		}
	}
}
