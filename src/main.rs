#[macro_use]
extern crate diesel;
extern crate dotenv;

use anyhow::Result;
use relm4::{adw, gtk, RelmApp};
use widgets::app::AppModel;
use crate::adw::prelude::ApplicationExt;

use crate::config::{load_css, set_app};

mod config;
mod models;
mod schema;
mod services;
mod storage;
mod widgets;

fn main() -> Result<()> {
    let application = adw::Application::builder()
        .application_id("do.edfloreshz.github")
        .flags(gtk::gio::ApplicationFlags::HANDLES_OPEN)
        .build();
    application.connect_startup(|_| load_css());
    set_app()?;
    let model = AppModel::new("");
    let app = RelmApp::with_app(model, application);
    app.run();
    Ok(())
}
