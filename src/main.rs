use anyhow::Result;

use gtk::gio;
use gtk::prelude::ApplicationExt;
use once_cell::unsync::Lazy;
use relm4::{
	actions::{AccelsPlus, RelmAction, RelmActionGroup},
	adw, gtk, RelmApp,
};

use app::App;
use once_cell::sync::OnceCell;
use setup::setup;
use tokio::runtime::Runtime;

use crate::config::APP_ID;

#[rustfmt::skip]
mod config;
mod app;
mod application;
mod setup;
mod widgets;

relm4::new_action_group!(AppActionGroup, "app");
relm4::new_stateless_action!(QuitAction, AppActionGroup, "quit");

thread_local! {
	static APP: Lazy<adw::Application> = Lazy::new(|| { adw::Application::new(Some(APP_ID), gio::ApplicationFlags::empty())});
}

static RT: OnceCell<Runtime> = OnceCell::new();

pub fn rt<'a>() -> &'a Runtime {
	RT.get().unwrap()
}

fn main_app() -> adw::Application {
	APP.with(|app| (*app).clone())
}

#[tokio::main]
async fn main() -> Result<()> {
	setup()?;

	let app = main_app();
	app.set_resource_base_path(Some("/dev/edfloreshz/Done/"));
	RT.set(Runtime::new().unwrap()).unwrap();
	let actions = RelmActionGroup::<AppActionGroup>::new();

	let quit_action = {
		let app = app.clone();
		RelmAction::<QuitAction>::new_stateless(move |_| {
			app.quit();
		})
	};

	actions.add_action(&quit_action);

	app.set_accelerators_for_action::<QuitAction>(&["<Control>q"]);

	app.set_action_group(Some(&actions.into_action_group()));
	let app = RelmApp::with_app(app);
	app.run::<App>(());
	Ok(())
}
