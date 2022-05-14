#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use anyhow::Result;
use diesel_migrations::embed_migrations;
use glib::Cast;
use relm4::{adw, gtk, gtk::gio, RelmApp};

use app::application::DoneApplication;
use app::config::{load_css, verify_data_integrity};
use widgets::app::AppModel;

use crate::adw::prelude::ApplicationExt;

mod app;
mod core;
mod models;
mod schema;
mod storage;
mod widgets;

embed_migrations!("migrations");

fn main() -> Result<()> {
    let application =
        DoneApplication::new("dev.edfloreshz.Done", &gio::ApplicationFlags::HANDLES_OPEN);
    application.connect_startup(|_| load_css());
    verify_data_integrity()?;
    let app = RelmApp::with_app(AppModel::new(), application.upcast());
    app.run();
    Ok(())
}
