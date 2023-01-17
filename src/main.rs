extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use anyhow::Result;
use relm4::{adw, gtk, RelmApp};

use app::App;
use setup::setup_app;

#[rustfmt::skip]
mod config;
mod app;
mod application;
mod setup;
mod widgets;

#[tokio::main]
async fn main() -> Result<()> {
	let app = RelmApp::with_app(setup_app().await?);
	app.run::<App>(());
	Ok(())
}
