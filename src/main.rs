#[macro_use]
extern crate diesel;
extern crate dotenv;

use relm4::{adw, gtk, RelmApp};
use widgets::app::AppModel;
use anyhow::Result;

mod widgets;
mod storage;
mod services;
mod schema;

fn main() -> Result<()> {
    let application = adw::Application::builder()
        .application_id("do.edfloreshz.github")
        .flags(gtk::gio::ApplicationFlags::HANDLES_OPEN)
        .build();
    let model = AppModel::default();
    let app = RelmApp::with_app(model, application);
    app.run();
    Ok(())
}
