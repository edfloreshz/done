use super::appearance;
use crate::application::{actions, gettext, localization, resources, settings};
use anyhow::Result;
use done_local_storage::service::Service;
use relm4::gtk::prelude::{
	ApplicationExt, ApplicationExtManual, ButtonExt, FileExt, GtkWindowExt,
	WidgetExt,
};
use relm4::{adw, gtk, view};
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

pub fn connect_signals(app: &adw::Application) {
	app.connect_activate(|app| {
		view! {
			window = &adw::ApplicationWindow {
				set_application: Some(app),
				set_default_width: 600,
				set_default_height: 700,
				set_width_request: 600,
				set_height_request: 700,
				gtk::Box {
					gtk::Button {
						connect_clicked => |_| {
							Service::Microsoft.get_service().login().unwrap();
						}
					}
				}
			}
		}
		window.show();
	});

	app.connect_open(|_, files, _| {
		let bytes = files[0].uri();
		let uri = reqwest::Url::from_str(bytes.to_string().as_str()).unwrap();
		tracing::info!("connect_open working... URI: {uri}");
		// relm4::tokio::spawn(async move {
		// 	let pairs = uri.query_pairs();
		// 	let response = Service::Microsoft
		// 		.get_service()
		// 		.handle_uri_params(pairs)
		// 		.await;
		// 	match response {
		// 		Ok(_) => tracing::info!("Token stored"),
		// 		Err(err) => tracing::error!("An error ocurred: {}", err),
		// 	}
		// });
	});
}

pub async fn init_services() -> Result<()> {
	settings::init().await?;
	appearance::init()?;
	Ok(())
}
