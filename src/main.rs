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
use gettextrs::{bind_textdomain_codeset, bindtextdomain, setlocale, textdomain, LocaleCategory};

use crate::app::constants::{GETTEXT_PACKAGE, LOCALEDIR};
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

	// Set up gettext translations
    debug!("Setting up locale data");
    setlocale(LocaleCategory::LcAll, "");

    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
        .expect("Unable to set the text domain encoding");
    textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

	load_resources()?;
	let application = DoneApplication::new();
	application.connect_startup(|_| load_css());
	verify_data_integrity()?;
	let app: RelmApp<AppModel> =
		RelmApp::with_app(application.upcast::<Application>());
	app.run(None);
	Ok(())
}
