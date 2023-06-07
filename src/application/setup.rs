use super::appearance;
use crate::app::Event;
use crate::application::{actions, gettext, localization, resources, settings};
use anyhow::Result;
use done_local_storage::service::Service;
use relm4::gtk::prelude::{ApplicationExtManual, FileExt};
use relm4::{adw, gtk, RelmApp};
use std::str::FromStr;

pub fn init(app: &adw::Application) -> Result<()> {
	gtk::init()?;
	gettext::init();
	localization::init();
	tracing_subscriber::fmt()
		.with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
		.with_max_level(tracing::Level::INFO)
		.init();
	resources::init()?;
	relm4_icons::initialize_icons();
	actions::init(app);

	Ok(())
}

pub fn connect_signals(app: RelmApp<Event>) -> RelmApp<Event> {
	app.on_activate(|app| {
		app.connect_open(|_, files, _| {
			let bytes = files[0].uri();
			let uri = reqwest::Url::from_str(bytes.to_string().as_str()).unwrap();
			relm4::tokio::spawn(async move {
				let pairs = uri.query_pairs();
				let response = Service::Microsoft
					.get_service()
					.handle_uri_params(pairs)
					.await;
				match response {
					Ok(_) => tracing::info!("Token stored"),
					Err(err) => tracing::error!("An error ocurred: {}", err),
				}
			});
		});
	})
}

pub async fn init_services() -> Result<()> {
	settings::init().await?;
	appearance::init()?;
	Ok(())
}
