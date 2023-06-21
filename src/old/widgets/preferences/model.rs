use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Preferences {
	pub color_scheme: ColorScheme,
	pub extended: bool,
}

impl Preferences {
	pub async fn new() -> Self {
		Self {
			color_scheme: ColorScheme::Default,
			extended: true,
		}
	}
}

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
