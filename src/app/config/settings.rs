use anyhow::Result;
use libset::{Config, FileType};

use done_core::service::Services;

use super::{info::APP_ID, preferences::Preferences};

pub(crate) fn init() -> Result<()> {
	migrate_old_database()?;
	ensure_app_config_exists()?;
	Services::init(APP_ID);
	Ok(())
}

fn ensure_app_config_exists() -> Result<()> {
	let app_config = Config::new(APP_ID, 1, None)?;
	if !app_config.path("preferences", FileType::Json)?.exists() {
		app_config.set_json("preferences", Preferences::new())?;
	}
	Ok(())
}

fn migrate_old_database() -> Result<()> {
	let database_config = Config::new(APP_ID, 1, Some("database"))?;

	let database_path =
		database_config.path(&format!("{APP_ID}.db"), FileType::Plain)?;

	let old_database_path = dirs::data_dir()
		.unwrap()
		.join("done")
		.join("dev.edfloreshz.Done.db");

	if old_database_path.exists() {
		let db = std::fs::read(&old_database_path)?;
		std::fs::write(database_path, db)?;
		std::fs::remove_dir_all(old_database_path.parent().unwrap())?;
	}
	Ok(())
}

pub(crate) fn refresh() -> Result<()> {
	Config::new(APP_ID, 1, None)?.clean()?;
	init()
}
