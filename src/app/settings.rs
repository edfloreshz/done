use iced::Font;

use super::ui::window;

#[derive(Debug, Default)]
pub struct Config {}

pub fn setup(config_load: Config) -> iced::Settings<Config> {
	iced::Settings {
		default_font: Font::DEFAULT,
		default_text_size: 16.0,
		window: window::settings(),
		exit_on_close_request: false,
		flags: config_load,
		id: None,
		antialiasing: false,
	}
}
