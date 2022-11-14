use anyhow::Result;
use gettextrs::{gettext, LocaleCategory};
use gtk::{gdk, gio, glib};
use libset::{config::Config, new_dir, new_file};
use relm4::gtk;
use relm4::actions::{RelmAction, RelmActionGroup};
use relm4::gtk::prelude::ApplicationExt;
use relm4::actions::AccelsPlus;
use relm4::adw;
use crate::{
	application::fluent::setup_fluent,
	config::{APP_ID, GETTEXT_PACKAGE, LOCALEDIR, VERSION},
};
use once_cell::unsync::Lazy;

relm4::new_action_group!(AppActionGroup, "app");
relm4::new_stateless_action!(QuitAction, AppActionGroup, "quit");

thread_local! {
    static APP: Lazy<adw::Application> = Lazy::new(|| { adw::Application::new(Some(APP_ID), gio::ApplicationFlags::empty())});
}

pub fn main_app() -> adw::Application {
    APP.with(|app| (*app).clone())
}

pub fn setup_app() -> Result<adw::Application> {
	gtk::init().unwrap();
	setup_gettext();
	setup_fluent()?;
	verify_data_integrity()?;
	pretty_env_logger::init();

	glib::set_application_name(&gettext("Done"));
	gio::resources_register_include!("resources.gresource")?;
	setup_css();
	gtk::Window::set_default_icon_name(APP_ID);

    let app = main_app();

    setup_actions(&app);

    Ok(app)
}

fn setup_actions(app: &adw::Application) {
    app.set_resource_base_path(Some("/dev/edfloreshz/Done/"));
    let actions = RelmActionGroup::<AppActionGroup>::new();

    let quit_action = {
        let app = app.clone();
        RelmAction::<QuitAction>::new_stateless(move |_| {
            app.quit();
        })
    };

    actions.add_action(&quit_action);

    app.set_accelerators_for_action::<QuitAction>(&["<Control>q"]);

    app.set_action_group(Some(&actions.into_action_group()));
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
