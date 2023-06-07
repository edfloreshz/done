use anyhow::Result;
use application::{info::APP_ID, setup};
use relm4::{adw, gtk::prelude::ApplicationExtManual, RelmApp};

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
	setup::connect_signals(&app);
	app.run();
	// let app = RelmApp::from_app(app).on_activate(|app| {
	// 	app.connect_open(|_, files, _| {
	// 		println!("Testing...");
	// 	});
	// });
	// app.run_async::<App>(());
	Ok(())
}
