use adw::prelude::ApplicationExtManual;
use gtk4 as gtk;
use libadwaita as adw;

use crate::data::app::App;

mod data;
mod events;
mod models;
mod services;
mod ui;

fn main() -> anyhow::Result<()> {
    let application = adw::Application::builder()
        .application_id("do.edfloreshz.github")
        .flags(gtk::gio::ApplicationFlags::HANDLES_OPEN)
        .build();
    App::connect_events(&application)?;
    application.run();
    Ok(())
}
