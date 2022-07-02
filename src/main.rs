#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use anyhow::Result;
use diesel_migrations::embed_migrations;
use relm4::adw::prelude::ApplicationExt;
use relm4::gtk::prelude::Cast;
use relm4::{adw, gtk, gtk::gio, RelmApp};

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
	let application = DoneApplication::new(
		"dev.edfloreshz.Done",
		&gio::ApplicationFlags::HANDLES_OPEN,
	);
	application.connect_startup(|_| load_css());
	verify_data_integrity()?;
	let app: RelmApp<AppModel> =
		RelmApp::with_app(application.upcast::<Application>());
	app.run(None);
	Ok(())
}
