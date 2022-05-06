use crate::adw::gdk::Display;
use crate::storage::database::DatabaseConnection;
use anyhow::Result;
use gtk4::{CssProvider, StyleContext};
use libset::config::Config;
use libset::fi;
use crate::embedded_migrations;

pub fn set_debug_options() -> Result<()> {
    let config = get_config();
    let user_database = dirs::data_dir().unwrap().join("do/org.devloop.Do.db");
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
    provider.load_from_data(include_bytes!("themes/Adwaita.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn get_config() -> Config {
    Config::new("do")
        .about("Do is a To Do app for Linux built with Rust and GTK.")
        .author("Eduardo Flores")
        .version("0.1.0")
        .add(fi!("org.devloop.Do.db"))
}
