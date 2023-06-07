use anyhow::Result;
use application::{info::APP_ID, setup};
use relm4::{gtk, RelmApp};

use app::App;

mod app;
mod application;
mod factories;
mod widgets;

fn main() -> Result<()> {
	let app = RelmApp::new(APP_ID);
	setup::init()?;
	app.run_async::<App>(());
	Ok(())
}
