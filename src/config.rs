use crate::adw::gdk::Display;
use anyhow::{Context, Result};

use gtk4::{CssProvider, StyleContext};
use libset::config::Config;
use libset::fi;
use std::io::Write;

pub fn set_debug_options() -> Result<()> {
    set_dotenv()?;
    Ok(())
}

fn set_dotenv() -> Result<()> {
    let mut file = std::fs::OpenOptions::new().write(true).open(".env")?;
    let data = dirs::data_dir().with_context(|| "")?;
    let data = data.join("do/org.devloop.Do.db");
    let url = format!(
        "DATABASE_URL={}",
        data.display().to_string().replace(' ', "\\ ")
    );
    file.write_all(url.as_bytes())?;
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

