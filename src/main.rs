use anyhow::Result;
use application::setup;
use relm4::{adw, gtk, RelmApp};

use app::App;

mod app;
mod application;
mod widgets;

fn main() -> Result<()> {
	let main_app = setup::init()?;
	let app = RelmApp::from_app(main_app);
	app.run_async::<App>(());
	Ok(())
}
