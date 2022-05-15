use anyhow::Result;
use libset::config::Config;
use libset::fi;
use relm4::gtk::{CssProvider, StyleContext};

use crate::adw::gdk::Display;
use crate::embedded_migrations;
use crate::storage::DatabaseConnection;

pub fn verify_data_integrity() -> Result<()> {
    let config = get_config();
    let user_database = dirs::data_dir()
        .unwrap()
        .join("done/dev.edfloreshz.Done.db");
    if !config.is_written() || !user_database.exists() {
        config.write()?;
    }
    let connection = DatabaseConnection::establish_connection();
    embedded_migrations::run(&connection)?;
    Ok(())
}

pub fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("../themes/Adwaita.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        relm4::gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn get_config() -> Config {
    Config::new("done")
        .about("Do is a To Do app for Linux built with Rust and GTK.")
        .author("Eduardo Flores")
        .version("0.1.0")
        .add(fi!("dev.edfloreshz.Done.db"))
}
