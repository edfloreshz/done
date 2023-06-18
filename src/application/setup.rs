use super::appearance;
use crate::application::{actions, gettext, localization, resources, settings};
use anyhow::Result;
use relm4::gtk::gio::ApplicationFlags;
use relm4::gtk::prelude::{ApplicationExt, ApplicationExtManual};
use relm4::{gtk, main_adw_application};

pub fn init() -> Result<()> {
	gtk::init()?;
	gettext::init();
	localization::init();
	tracing_subscriber::fmt()
		.with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
		.with_max_level(tracing::Level::INFO)
		.init();
	resources::init()?;
	relm4_icons::initialize_icons();
	actions::init();

	connect_signals();

	Ok(())
}

pub fn connect_signals() {
	let app = main_adw_application();
	app.set_flags(ApplicationFlags::HANDLES_OPEN);
	app.connect_open(|_, _, _| {});
}

pub async fn init_services() -> Result<()> {
	settings::init().await?;
	appearance::init()?;
	Ok(())
}
