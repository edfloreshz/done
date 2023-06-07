use anyhow::Result;
use application::{info::APP_ID, setup};
use done_local_storage::service::Service;
use relm4::{
	adw,
	gtk::prelude::{ApplicationExtManual, FileExt},
	RelmApp,
};
use std::str::FromStr;

use app::App;

mod app;
mod application;
mod factories;
mod widgets;

fn main() -> Result<()> {
	let app = adw::Application::builder()
		.application_id(APP_ID)
		.flags(adw::gio::ApplicationFlags::HANDLES_OPEN)
		.build();
	setup::init(&app)?;
	let app = RelmApp::from_app(app).on_activate(|app| {
		app.connect_open(|_, files, _| {
			println!("Testing...");
		});
	});
	app.run_async::<App>(());
	Ok(())
}
