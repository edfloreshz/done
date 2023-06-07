use std::str::FromStr;

use crate::application::{actions, gettext, localization, resources, settings};
use anyhow::Result;
use done_local_storage::service::Service;
use relm4::{
	adw,
	gtk::{
		self,
		gio::ApplicationFlags,
		prelude::{ApplicationExtManual, FileExt},
	},
};

use super::{appearance, info::APP_ID};

pub fn init() -> Result<adw::Application> {
	let app = adw::Application::builder()
		.application_id(APP_ID)
		.flags(ApplicationFlags::HANDLES_OPEN)
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

	app.connect_open(|_, files, _| {
		println!("Yay!");
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

	actions::init(&app);

	Ok(app)
}

pub async fn init_services() -> Result<()> {
	settings::init().await?;
	appearance::init()?;
	Ok(())
}
