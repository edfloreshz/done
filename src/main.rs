use anyhow::Result;
use relm4::{adw, gtk, RelmApp};

use app::App;

mod app;
mod application;
mod helpers;
mod widgets;

fn main() -> Result<()> {
	let app = RelmApp::with_app(app::main_app());
	app.run_async::<App>(());
	Ok(())
}
