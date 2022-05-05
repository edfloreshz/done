use crate::adw::gdk::Display;
use crate::storage::database::DatabaseConnection;
use anyhow::Result;
use diesel_migrations::{
    any_pending_migrations, run_pending_migrations,
};
use gtk4::{CssProvider, StyleContext};
use libset::config::Config;
use libset::fi;
use std::io::Write;
use std::path::PathBuf;

const DEBUG_MODE: bool = cfg!(debug_assertions);

pub fn set_debug_options() -> Result<()> {
    let config = get_config();
    let user_database = dirs::data_dir().unwrap().join("do/org.devloop.Do.db");
    let system_database = if DEBUG_MODE {
        dirs::home_dir().unwrap().join(".local/share/flatpak/app/org.devloop.Do/current/active/files/share/data/org.devloop.Do.db")
    } else {
        PathBuf::from("/var/lib/flatpak/app/org.devloop.Do/current/active/files/share/data/org.devloop.Do.db")
    };
    if !config.is_written() || !user_database.exists() {
        config.write()?;
        std::fs::copy(system_database, user_database)?;
    }
    if DEBUG_MODE {
        set_dotenv()?;

        let connection = DatabaseConnection::establish_connection();

        if any_pending_migrations(&connection)? {
            run_pending_migrations(&connection)?;
        }
    }
    Ok(())
}

fn set_dotenv() -> Result<()> {
    let mut file = std::fs::OpenOptions::new().write(true).open(".env")?;
    let data = dirs::data_dir().unwrap();
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
