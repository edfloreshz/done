use anyhow::{Context, Result};
use diesel::{Connection, SqliteConnection};
use gettextrs::{gettext, LocaleCategory};
use gtk::{gdk, gio, glib};
use libset::{config::Config, new_dir, new_file};
use relm4::gtk;

use crate::{
	application::fluent::setup_fluent,
	config::{APP_ID, GETTEXT_PACKAGE, LOCALEDIR, VERSION},
	embedded_migrations,
};

pub fn setup() -> Result<()> {
	// Initialize GTK
	gtk::init().unwrap();

	// Initialize logger
	pretty_env_logger::init();

	setup_gettext();
	setup_fluent()?;

	glib::set_application_name(&gettext("Done"));

	gio::resources_register_include!("resources.gresource")?;

	setup_css();
	verify_data_integrity()?;

	gtk::Window::set_default_icon_name(APP_ID);

	Ok(())
}

fn setup_gettext() {
	// Prepare i18n
	gettextrs::setlocale(LocaleCategory::LcAll, "");
	gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR)
		.expect("Unable to bind the text domain");
	gettextrs::textdomain(GETTEXT_PACKAGE)
		.expect("Unable to switch to the text domain");
}

fn setup_css() {
	let provider = gtk::CssProvider::new();
	provider.load_from_resource("/dev/edfloreshz/Done/ui/style.css");
	if let Some(display) = gdk::Display::default() {
		gtk::StyleContext::add_provider_for_display(
			&display,
			&provider,
			gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
		);
	}
}

pub fn verify_data_integrity() -> Result<()> {
	let config = get_config();
	let user_database = dirs::data_dir()
		.unwrap()
		.join("done/dev.edfloreshz.Done.db");
	if
	// !config.is_written() ||
	!user_database.exists() {
		config.write()?;
	}
	let database_path = dirs::data_dir()
		.with_context(|| "Failed to get data directory.")?
		.join("done/dev.edfloreshz.Done.db");
	let database_url = database_path
		.to_str()
		.with_context(|| "Failed to convert path to string")?;
	let connection = SqliteConnection::establish(database_url)
		.with_context(|| "Error connecting to database")?;
	embedded_migrations::run(&connection)?;
	Ok(())
}

fn get_config() -> Config {
	Config::new("done")
		.about("Do is a To Do app for Linux built with Rust and GTK.")
		.author("Eduardo Flores")
		.version(VERSION)
		.add(new_file!("dev.edfloreshz.Done.db"))
		.add(new_dir!("providers"))
}
