use anyhow::Result;
use libset::{Config, FileType};

use done_core::services::local::database::{Database, DATABASE_NAME};

use super::{info::APP_ID, preferences::Preferences};

pub(crate) fn init() -> Result<()> {
	let config = Config::new(APP_ID, 1, None)?;
	let database = Config::new(APP_ID, 1, Some("database"))?;
	let database_path = database.path(DATABASE_NAME, FileType::Plain)?;

	let previous_database_path = dirs::data_dir()
		.unwrap()
		.join("done")
		.join("dev.edfloreshz.Done.db");

	if previous_database_path.exists() {
		let db = std::fs::read(&previous_database_path)?;
		std::fs::write(&database_path, db)?;
		std::fs::remove_dir_all(previous_database_path.parent().unwrap())?;
	}

	if !config.path("preferences", FileType::Json)?.exists() {
		config.set_json("preferences", Preferences::new())?;
	}
	if !database_path.exists() {
		database.set_plain(DATABASE_NAME, String::new())?;
	}

	Database::ensure_migrations_up_to_date()?;

	Ok(())
}

pub(crate) fn refresh() -> Result<()> {
	Config::new(APP_ID, 1, None)?.clean()?;
	init()
}
