#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use anyhow::Result;

use gtk::gio;
use gtk::prelude::ApplicationExt;
use once_cell::{sync::Lazy as LazySync, unsync::Lazy};
use relm4::{
	actions::{AccelsPlus, RelmAction, RelmActionGroup},
	adw, gtk, RelmApp,
};


use app::App;
use setup::setup;

use crate::config::APP_ID;
use crate::data::plugins::Plugins;
use crate::data::traits::provider::Provider;

#[rustfmt::skip]
mod config;
mod app;
mod application;
mod data;
mod schema;
mod setup;
mod widgets;

relm4::new_action_group!(AppActionGroup, "app");
relm4::new_stateless_action!(QuitAction, AppActionGroup, "quit");

static PLUGINS: LazySync<Plugins> = LazySync::new(Plugins::default);

thread_local! {
	static APP: Lazy<adw::Application> = Lazy::new(|| { adw::Application::new(Some(APP_ID), gio::ApplicationFlags::empty())});
}

embed_migrations!("migrations");

pub type StaticProviderType = &'static dyn Provider;

fn main_app() -> adw::Application {
	APP.with(|app| (*app).clone())
}

fn main() -> Result<()> {
	setup()?;

	let app = main_app();
	app.set_resource_base_path(Some("/dev/edfloreshz/Done/"));

	let actions = RelmActionGroup::<AppActionGroup>::new();

	let quit_action = {
		let app = app.clone();
		RelmAction::<QuitAction>::new_stateless(move |_| {
			app.quit();
		})
	};

	actions.add_action(quit_action);

	app.set_accelerators_for_action::<QuitAction>(&["<Control>q"]);

	app.set_action_group(Some(&actions.into_action_group()));
	let app = RelmApp::with_app(app);
	app.run::<App>(None);
	Ok(())
}
