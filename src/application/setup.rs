use std::str::FromStr;

use crate::application::{
	actions, gettext, info::APP_ID, localization, resources, settings,
};
use anyhow::{Ok, Result};
use done_local_storage::service::Service;
use once_cell::unsync::Lazy;
use relm4::{
	adw,
	gtk::{
		self,
		prelude::{ApplicationExtManual, FileExt},
	},
	gtk::{gio, prelude::ApplicationExt, traits::WidgetExt},
	view,
};

use super::appearance;

pub fn init(app: adw::Application) -> Result<adw::Application> {
	app.connect_open(|_, files, _| {
		let bytes = files[0].uri();
		let uri = reqwest::Url::from_str(bytes.to_string().as_str()).unwrap();
		let pairs = uri.query_pairs().next().unwrap().1;
		println!("{}", pairs.to_string());
	});
	use relm4::gtk::prelude::ButtonExt;
	use relm4::gtk::prelude::GtkWindowExt;
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
								connect_clicked => |s| {
									Service::Microsoft.get_service().login().unwrap();
								}
							}
						}
				}
		}
		window.show();
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
