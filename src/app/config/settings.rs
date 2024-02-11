use anyhow::Result;
use core_done::services::local::database::{Database, DATABASE_NAME};
use libset::{Config, FileType};

use super::preferences::Preferences;

pub(crate) fn init() -> Result<()> {
	let config = Config::new("dev.edfloreshz.done", 1, None)?;
	let database = Config::new("dev.edfloreshz.done", 1, Some("database"))?;
	if !config.path("preferences", FileType::Json)?.exists() {
		config.set_json("preferences", Preferences::new())?;
	}
	if !database.path(DATABASE_NAME, FileType::Plain)?.exists() {
		database.set_plain(DATABASE_NAME, String::new())?;
	}

	Database::ensure_migrations_up_to_date()?;

	Ok(())
}

pub(crate) fn refresh() -> Result<()> {
	Config::new("dev.edfloreshz.done", 1, None)?.clean()?;
	init()
}
