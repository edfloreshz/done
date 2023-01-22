use crate::application::{
	actions, gettext, localization, resources, services, settings,
};
use anyhow::{Ok, Result};
use relm4::gtk;

use super::appearance;

pub fn init_app() -> Result<()> {
	gtk::init()?;
	gettext::init();
	localization::init();
	appearance::init()?;
	// Enable logging
	tracing_subscriber::fmt()
		.with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
		.with_max_level(tracing::Level::INFO)
		.init();
	resources::init()?;
	actions::init();

	Ok(())
}

pub async fn init_services() -> Result<()> {
	settings::init().await?;
	services::init().await?;
	Ok(())
}
