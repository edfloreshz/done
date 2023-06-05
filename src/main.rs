use anyhow::Result;
use application::{info::APP_ID, setup};
use relm4::{adw, gtk, RelmApp};

use app::App;

mod app;
mod application;
mod factories;
mod widgets;

fn main() -> Result<()> {
	let application = adw::Application::builder()
		.application_id(APP_ID)
		.flags(gtk::gio::ApplicationFlags::HANDLES_OPEN)
		.build();
	let main_app = setup::init(application)?;
	let app = RelmApp::from_app(main_app);
	app.run_async::<App>(());
	Ok(())
}
