use crate::application::{actions, gettext, localization, resources, settings};
use anyhow::{Ok, Result};
use relm4::{adw, gtk};

use super::{appearance, info::APP_ID};

pub fn init() -> Result<adw::Application> {
	let app = adw::Application::builder()
		.application_id(APP_ID)
		.flags(gtk::gio::ApplicationFlags::HANDLES_OPEN)
		.build();

	gtk::init()?;
	gettext::init();
	localization::init();
	tracing_subscriber::fmt()
		.with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
		.with_max_level(tracing::Level::INFO)
		.init();
	resources::init()?;
	relm4_icons::initialize_icons();
	actions::init(&app);

	Ok(app)
}

pub async fn init_services() -> Result<()> {
	settings::init().await?;
	appearance::init()?;
	Ok(())
}
