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
mod application;
mod constants;
mod window;

use anyhow::Result;
use relm4::{adw, gtk, RelmApp};
use widgets::app::AppModel;
use diesel_migrations::embed_migrations;

use crate::adw::prelude::ApplicationExt;
use crate::application::DoneApplication;
use crate::config::{load_css, set_debug_options};

embed_migrations!("migrations");

fn main() -> Result<()> {
    let application = DoneApplication::new("dev.edfloreshz.Done", &gtk::gio::ApplicationFlags::HANDLES_OPEN);
    application.connect_startup(|_| load_css());
    set_debug_options()?;
    let app = RelmApp::with_app(AppModel::new(), application);
    app.run();
    Ok(())
}
