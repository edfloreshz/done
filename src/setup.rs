use crate::{
	application::{fluent::setup_fluent, plugin::Plugin},
	config::{APP_ID, GETTEXT_PACKAGE, LOCALEDIR, VERSION},
	widgets::components::preferences::Preferences,
};
use anyhow::{Ok, Result};
use gettextrs::{gettext, LocaleCategory};
use gtk::{gdk, gio, glib};
use libset::{format::FileFormat, new_file, project::Project};
use once_cell::unsync::Lazy;
use relm4::actions::AccelsPlus;
use relm4::actions::{RelmAction, RelmActionGroup};
use relm4::adw;
use relm4::gtk;
use relm4::gtk::prelude::ApplicationExt;
use sysinfo::{ProcessExt, System, SystemExt};

relm4::new_action_group!(AppActionGroup, "app");
relm4::new_stateless_action!(QuitAction, AppActionGroup, "quit");

thread_local! {
		static APP: Lazy<adw::Application> = Lazy::new(|| { adw::Application::new(Some(APP_ID), gio::ApplicationFlags::empty())});
}

pub fn main_app() -> adw::Application {
	APP.with(|app| (*app).clone())
}

pub async fn setup_app() -> Result<adw::Application> {
	gtk::init()?;
	setup_gettext();
	setup_fluent()?;
	verify_data_integrity().await?;
	pretty_env_logger::init();

	glib::set_application_name(&gettext("Done"));
	gio::resources_register_include!("resources.gresource")?;
	setup_css();
	gtk::Window::set_default_icon_name(APP_ID);

	start_services().await?;

	let app = main_app();

	app.connect_shutdown(|_| {
		let processes = System::new_all();
		let mut local = processes.processes_by_exact_name("local-plugin");
		match local.next() {
			Some(process) => {
				if process.kill() {
					info!("The {} process was killed.", process.name())
				} else {
					error!("Failed to kill process.")
				}
			},
			None => info!("Process is not running."),
		}
	});

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

pub async fn verify_data_integrity() -> Result<()> {
	let project = Project::new("dev", "edfloreshz", "done")
		.about("Done is a simple to do app.")
		.author("Eduardo Flores")
		.version(VERSION)
		.add_files(&[
			new_file!("preferences").set_format(FileFormat::JSON),
			new_file!("dev.edfloreshz.Done.Plugins").set_format(FileFormat::JSON),
			new_file!("dev.edfloreshz.Done.db").set_format(FileFormat::Plain),
		])?;
	if !project.integrity_ok::<Preferences>("preferences", FileFormat::JSON) {
		project
			.get_file("preferences", FileFormat::JSON)?
			.set_content(Preferences::default())?
			.write()?;
	}
	let plugins: Vec<Plugin> = Plugin::fetch_plugins().await?;
	project
		.get_file("dev.edfloreshz.Done.Plugins", FileFormat::JSON)?
		.set_content(plugins)?
		.write()?;
	Ok(())
}

async fn start_services() -> Result<()> {
	for plugin in Plugin::fetch_plugins().await? {
		if !plugin.is_running() {
			if let Err(e) = plugin.start() {
				info!("{:?}", e)
			};
		}
	}
	Ok(())
}
