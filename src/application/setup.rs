use crate::application::{
	actions, gettext, info::APP_ID, localization, resources, settings,
};
use anyhow::{Ok, Result};
use once_cell::unsync::Lazy;
use relm4::{adw, gtk, gtk::gio};

use super::appearance;

thread_local! {
	static APP: Lazy<adw::Application> = Lazy::new(|| { adw::Application::new(Some(APP_ID), gio::ApplicationFlags::empty())});
}

pub fn main_app() -> adw::Application {
	APP.with(|app| (*app).clone())
}

pub fn init() -> Result<adw::Application> {
	let app = main_app();

	gtk::init()?;
	gettext::init();
	localization::init();
	// Enable logging
	tracing_subscriber::fmt()
		.with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
		.with_max_level(tracing::Level::INFO)
		.init();
	resources::init()?;
	actions::init(&app);

	Ok(app)
}

pub async fn init_services() -> Result<()> {
	settings::init().await?;
	appearance::init()?;
	Ok(())
}
