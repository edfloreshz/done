use super::appearance;
use crate::application::{actions, gettext, localization, resources, settings};
use anyhow::Result;
use done_local_storage::service::Service;
use relm4::gtk::gio::ApplicationFlags;
use relm4::gtk::prelude::{ApplicationExt, ApplicationExtManual, FileExt};
use relm4::{gtk, main_adw_application};
use std::str::FromStr;

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
	app.connect_open(|_, files, _| {
		let bytes = files[0].uri();
		let uri = reqwest::Url::from_str(bytes.to_string().as_str()).unwrap();
		relm4::tokio::spawn(async move {
			let response = Service::Microsoft
				.get_service()
				.handle_uri_params(uri)
				.await;
			match response {
				Ok(_) => tracing::info!("Token stored"),
				Err(err) => tracing::error!("An error ocurred: {}", err),
			}
		});
	});
}

pub async fn init_services() -> Result<()> {
	settings::init().await?;
	appearance::init()?;
	Ok(())
}
