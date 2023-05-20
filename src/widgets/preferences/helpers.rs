use anyhow::Result;
use libset::{format::FileFormat, project::Project};
use relm4::{adw, AsyncComponentSender};

use crate::widgets::preferences::messages::PreferencesComponentOutput;

use super::model::{ColorScheme, Preferences, PreferencesComponentModel};

pub fn set_color_scheme(
	model: &mut PreferencesComponentModel,
	color_scheme: ColorScheme,
) -> Result<()> {
	match color_scheme {
		ColorScheme::Dark => {
			adw::StyleManager::default()
				.set_color_scheme(adw::ColorScheme::ForceDark);
			model.preferences.color_scheme = ColorScheme::Dark;
		},
		ColorScheme::Light => {
			adw::StyleManager::default()
				.set_color_scheme(adw::ColorScheme::ForceLight);
			model.preferences.color_scheme = ColorScheme::Light;
		},
		ColorScheme::Default => {
			adw::StyleManager::default().set_color_scheme(adw::ColorScheme::Default);
			model.preferences.color_scheme = ColorScheme::Default;
		},
	}
	update_preferences(&model.preferences)
}

pub fn set_extended(
	model: &mut PreferencesComponentModel,
	sender: &AsyncComponentSender<PreferencesComponentModel>,
	extended: bool,
) -> Result<()> {
	model.preferences.extended = extended;
	update_preferences(&model.preferences)?;
	sender
		.output(PreferencesComponentOutput::ToggleExtended(
			model.preferences.extended,
		))
		.unwrap();
	Ok(())
}

fn update_preferences(preferences: &Preferences) -> Result<()> {
	Project::open("dev", "edfloreshz", "done")?
		.get_file("preferences", FileFormat::JSON)?
		.set_content(preferences)?
		.write()
}
