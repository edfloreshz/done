#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use anyhow::Result;
use diesel_migrations::embed_migrations;
use relm4::adw::prelude::ApplicationExt;
use relm4::gtk::prelude::Cast;
use relm4::{adw, gtk, gtk::gio, RelmApp};
use crate::app::resources::load_resources;

use crate::adw::Application;
use widgets::app::AppModel;

use crate::app::application::DoneApplication;
use crate::app::config::{load_css, verify_data_integrity};

mod app;
mod core;
mod models;
mod schema;
mod storage;
mod widgets;

embed_migrations!("migrations");

fn main() -> Result<()> {
	pretty_env_logger::init();
	load_resources()?;
	let application = DoneApplication::new();
	application.connect_startup(|_| load_css());
	verify_data_integrity()?;
	let app: RelmApp<AppModel> =
		RelmApp::with_app(application.upcast::<Application>());
	app.run(None);
	Ok(())
}
