use app::{
	logger, notifications,
	settings::{self, Config},
	Done,
};
use iced::Application;

mod app;

fn main() -> iced::Result {
	logger::setup();
	notifications::setup();
	let settings = settings::setup(Config::default());
	Done::run(settings)
}
