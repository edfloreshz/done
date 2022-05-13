#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod app;
mod core;
mod models;
mod schema;
mod storage;
mod widgets;

use anyhow::Result;
use diesel_migrations::embed_migrations;
use glib::Cast;
use relm4::{adw, gtk, RelmApp};
use widgets::app::AppModel;

use crate::adw::prelude::ApplicationExt;
use app::application::DoneApplication;
use app::config::{load_css, set_debug_options};

embed_migrations!("migrations");

fn main() -> Result<()> {
    let application = DoneApplication::new(
        "dev.edfloreshz.Done",
        &gtk::gio::ApplicationFlags::HANDLES_OPEN,
    );
    application.connect_startup(|_| load_css());
    let application = application.upcast();
    set_debug_options()?;
    let app = RelmApp::with_app(AppModel::new(), application);
    app.run();
    Ok(())
}
