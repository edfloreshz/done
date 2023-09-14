use anyhow::Result;
use core_done::services::local::database::Database;
use libset::{format::FileFormat, new_file, project::Project};

use super::{info::VERSION, preferences::Preferences};

pub(crate) async fn init() -> Result<()> {
	let project = Project::new("dev", "edfloreshz", "done")
		.about("Done is a simple to do app.")
		.author("Eduardo Flores")
		.version(VERSION)
		.add_files(&[
			new_file!("preferences").set_format(FileFormat::JSON),
			new_file!("dev.edfloreshz.Done.db").set_format(FileFormat::Plain),
		])?;

	Database::ensure_migrations_up_to_date()?;

	if !project.integrity::<Preferences>("preferences", FileFormat::JSON) {
		project
			.get_file("preferences", FileFormat::JSON)?
			.set_content(Preferences::new().await)?
			.write()?;
	}
	Ok(())
}
