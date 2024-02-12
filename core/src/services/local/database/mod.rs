pub mod models;

use anyhow::{anyhow, Context, Result};
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{
	embed_migrations, EmbeddedMigrations, MigrationHarness,
};
use libset::Config;

use crate::services::microsoft::service::APP_ID;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
pub const DATABASE_NAME: &str = "dev.edfloreshz.Done.db";

pub struct Database;

impl Database {
	fn database_url() -> Result<String> {
		let url = Config::new(APP_ID, 1, Some("database"))?
			.path(DATABASE_NAME, libset::FileType::Plain)?;
		Ok(url.display().to_string())
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
