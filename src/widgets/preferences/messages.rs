use super::model::ColorScheme;

#[derive(Debug)]
pub enum PreferencesComponentInput {
	SetColorScheme(ColorScheme),
	ToggleCompact(bool),
	ToggleExtended(bool),
}

#[derive(Debug)]
pub enum PreferencesComponentOutput {
	ToggleCompact(bool),
	ToggleExtended(bool),
}
