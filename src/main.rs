#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod config;
mod core;
mod models;
mod schema;
mod storage;
mod widgets;

use anyhow::Result;
use relm4::{adw, gtk, RelmApp};
use widgets::app::AppModel;
use diesel_migrations::embed_migrations;

use crate::adw::prelude::ApplicationExt;
use crate::config::{load_css, set_debug_options};

embed_migrations!("migrations");

fn main() -> Result<()> {
    let application = adw::Application::builder()
        .application_id("dev.edfloreshz.ToDo")
        .flags(gtk::gio::ApplicationFlags::HANDLES_OPEN)
        .build();
    application.connect_startup(|_| load_css());
    set_debug_options()?;
    let model = AppModel::new();
    let app = RelmApp::with_app(model, application);
    app.run();
    Ok(())
}
