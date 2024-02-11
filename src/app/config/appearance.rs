use anyhow::Result;
use libset::Config;
use relm4::adw;
use serde::{Deserialize, Serialize};

use super::preferences::Preferences;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ColorScheme {
	Dark,
	Light,
	Default,
}

impl Default for ColorScheme {
	fn default() -> Self {
		Self::Default
	}
}

pub(crate) fn init() -> Result<()> {
	let project = Config::new("dev.edfloreshz.done", 1, None).unwrap();
	match project.get_json::<Preferences>("preferences") {
		Ok(preferences) => {
			let color_scheme = match preferences.color_scheme {
				ColorScheme::Dark => adw::ColorScheme::ForceDark,
				ColorScheme::Light => adw::ColorScheme::ForceLight,
				ColorScheme::Default => adw::ColorScheme::Default,
			};
			adw::StyleManager::default().set_color_scheme(color_scheme);
		},
		Err(err) => {
			tracing::error!("Failed to open settings: {}", err.to_string())
		},
	}
	Ok(())
}
