use std::str::FromStr;

use crate::application::{actions, gettext, localization, resources, settings};
use anyhow::{Ok, Result};
use done_local_storage::service::Service;
use relm4::{
	adw,
	gtk::{
		self,
		prelude::{ApplicationExtManual, FileExt},
	},
};

use super::appearance;

pub fn init(app: adw::Application) -> Result<adw::Application> {
	app.connect_open(|_, files, _| {
		let bytes = files[0].uri();
		let uri = reqwest::Url::from_str(bytes.to_string().as_str()).unwrap();
		relm4::tokio::spawn(async move {
			let pairs = uri.query_pairs();
			Service::Microsoft
				.get_service()
				.handle_uri_params(pairs)
				.await
				.unwrap();
		});
	});

	done_local_storage::setup::init()?;

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
