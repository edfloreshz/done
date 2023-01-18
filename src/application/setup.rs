use crate::application::{
	actions, gettext, info::APP_ID, localization, resources, services, settings,
};
use anyhow::{Ok, Result};
use once_cell::unsync::Lazy;
use relm4::{adw, gtk, gtk::gio};

thread_local! {
	static APP: Lazy<adw::Application> = Lazy::new(|| { adw::Application::new(Some(APP_ID), gio::ApplicationFlags::empty())});
}

pub fn main_app() -> adw::Application {
	APP.with(|app| (*app).clone())
}

pub async fn init() -> Result<adw::Application> {
	let app = main_app();

	gtk::init()?;
	gettext::init();
	localization::init();
	settings::init().await?;
	pretty_env_logger::init();
	resources::init()?;
	services::init().await?;
	actions::init(&app);

	Ok(app)
}
