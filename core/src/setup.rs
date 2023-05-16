use anyhow::Result;
use libset::{format::FileFormat, new_file, project::Project};

use crate::database::{Database, DATABASE_NAME};

pub fn init() -> Result<()> {
	Project::new("dev", "edfloreshz", "done")
		.author("Eduardo Flores <edfloreshz@gmail.com>")
		.about("Done local storage")
		.version("0.1.2")
		.add_files(&[new_file!(DATABASE_NAME).set_format(FileFormat::Plain)])?;

	Database::ensure_migrations_up_to_date()?;

	Ok(())
}
