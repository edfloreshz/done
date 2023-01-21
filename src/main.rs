use anyhow::Result;
use application::setup;
use relm4::{adw, gtk, RelmApp};
use std::time::Duration;

use app::App;

mod app;
mod application;
mod widgets;

fn main() -> Result<()> {
	let app = RelmApp::with_app(setup::init()?);
	let init_task = relm4::spawn(setup::async_init());
	while !init_task.is_finished() {
		std::thread::sleep(Duration::from_millis(50));
	}

	app.run::<App>(());
	Ok(())
}
