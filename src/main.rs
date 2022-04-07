#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::io::Write;
use relm4::{adw, gtk, RelmApp};
use widgets::app::AppModel;
use anyhow::{Context, Result};

mod widgets;
mod storage;
mod services;
mod schema;
pub mod models;

fn main() -> Result<()> {
    let application = adw::Application::builder()
        .application_id("do.edfloreshz.github")
        .flags(gtk::gio::ApplicationFlags::HANDLES_OPEN)
        .build();
    let model = AppModel::default();
    let app = RelmApp::with_app(model, application);
    set_dotenv()?;
    app.run();
    Ok(())
}

fn set_dotenv() -> Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .open(".env")?;
    let home = dirs::home_dir().with_context(|| "")?;
    let home = home.display();
    let cont = format!("DATABASE_URL={home}/.local/share/do/do.db");
    file.write_all(cont.as_bytes())?;
    Ok(())
}
