use anyhow::Result;
use gettextrs::{gettext, LocaleCategory};
use gtk::{gdk, gio, glib};
use libset::{config::Config, new_dir, new_file};
use relm4::gtk;

use crate::{
	application::fluent::setup_fluent,
	config::{APP_ID, GETTEXT_PACKAGE, LOCALEDIR, VERSION},
};

pub fn setup() -> Result<()> {
	gtk::init().unwrap();
	setup_gettext();
	setup_fluent()?;
	setup_css();
	verify_data_integrity()?;

	glib::set_application_name(&gettext("Done"));
	gio::resources_register_include!("resources.gresource")?;
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
	let app_data_dir = dirs::data_dir()
		.unwrap()
		.join("done");
	if !app_data_dir.exists() {
		config.write()?;
	}
	Ok(())
}

fn get_config() -> Config {
	Config::new("done")
		.about("Done is a To Do app for Linux built with Rust and GTK.")
		.author("Eduardo Flores")
		.version(VERSION)
		.add(new_file!("dev.edfloreshz.Done.db"))
		.add(new_dir!("providers"))
}
