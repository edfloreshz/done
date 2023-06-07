use anyhow::Result;
use application::setup;
use relm4::{gtk, RelmApp};

use app::App;

mod app;
mod application;
mod factories;
mod widgets;

fn main() -> Result<()> {
	let app = setup::init()?;
	let app = RelmApp::from_app(app);
	app.run_async::<App>(());
	Ok(())
}
