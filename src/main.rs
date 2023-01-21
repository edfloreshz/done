use anyhow::Result;
use application::setup;
use relm4::{adw, gtk, RelmApp};

use app::App;

mod app;
mod application;
mod widgets;

#[tokio::main]
async fn main() -> Result<()> {
	let app = RelmApp::with_app(setup::init().await?);
	app.run::<App>(());
	Ok(())
}
