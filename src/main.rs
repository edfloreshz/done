#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use anyhow::Result;
use diesel_migrations::embed_migrations;
use relm4::{adw, gtk, RelmApp};

use widgets::app::AppModel;

mod app;
mod core;
mod models;
mod schema;
mod storage;
mod widgets;

embed_migrations!("migrations");

fn main() -> Result<()> {
    let app: RelmApp<AppModel> = RelmApp::new("dev.edfloreshz.Done");
    app.run(None);
    Ok(())
}
