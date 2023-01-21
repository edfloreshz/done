use anyhow::Result;
use libset::{format::FileFormat, project::Project};
use relm4::adw;

use crate::widgets::components::preferences::{ColorScheme, Preferences};
pub(crate) fn init() -> Result<()> {
	let project = Project::open("dev", "edfloreshz", "done").unwrap();
	let preferences = project
		.get_file_as::<Preferences>("preferences", FileFormat::JSON)
		.unwrap();
	let color_scheme = match preferences.color_scheme {
		ColorScheme::Dark => adw::ColorScheme::ForceDark,
		ColorScheme::Light => adw::ColorScheme::ForceLight,
		ColorScheme::Default => adw::ColorScheme::Default,
	};
	adw::StyleManager::default().set_color_scheme(color_scheme);
	Ok(())
}
