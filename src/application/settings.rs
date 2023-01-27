use anyhow::Result;
use libset::{format::FileFormat, new_file, project::Project};

use crate::{
	application::info::VERSION, widgets::preferences::model::Preferences,
};

pub(crate) async fn init() -> Result<()> {
	let project = Project::new("dev", "edfloreshz", "done")
		.about("Done is a simple to do app.")
		.author("Eduardo Flores")
		.version(VERSION)
		.add_files(&[
			new_file!("preferences").set_format(FileFormat::JSON),
			new_file!("dev.edfloreshz.Done.Plugins").set_format(FileFormat::JSON),
		])?;

	if !project.integrity::<Preferences>("preferences", FileFormat::JSON) {
		project
			.get_file("preferences", FileFormat::JSON)?
			.set_content(Preferences::new().await)?
			.write()?;
	}
	Ok(())
}
