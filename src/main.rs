#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::io::Write;

use anyhow::{Context, Result};
use diesel_migrations::{any_pending_migrations, run_pending_migrations};
use libdmd::config::Config;
use libdmd::fi;
use relm4::{adw, gtk, MicroComponent, RelmApp};

use widgets::app::AppModel;

use crate::storage::database::DatabaseConnection;
use crate::widgets::content::ContentModel;

mod widgets;
mod storage;
mod services;
mod schema;
mod models;

fn main() -> Result<()> {
    let config = set_config();
    if !config.is_written() {
        config.write()?;
    }
    set_dotenv()?;
    let connection = DatabaseConnection::establish_connection();
    if any_pending_migrations(&connection)? {
        run_pending_migrations(&connection)?;
    }
    let application = adw::Application::builder()
        .application_id("do.edfloreshz.github")
        .flags(gtk::gio::ApplicationFlags::HANDLES_OPEN)
        .build();
    let model = AppModel::new(MicroComponent::new(ContentModel::default(), ()));
    let app = RelmApp::with_app(model, application);
    app.run();
    Ok(())
}

fn set_dotenv() -> Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .open(".env")?;
    let home = dirs::home_dir().with_context(|| "")?;
    let home = home.display();
    let url = format!("DATABASE_URL={home}/.local/share/do/do.db");
    file.write_all(url.as_bytes())?;
    Ok(())
}

fn set_config() -> Config {
    Config::new("do")
        .about("Do is a To Do app for Linux built with Rust and GTK.")
        .author("Eduardo Flores")
        .version("0.1.0")
        .add(fi!("do.db"))
}
