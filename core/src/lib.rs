#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use diesel_migrations::EmbeddedMigrations;

pub mod models;
pub mod traits;
pub mod enums;
mod data;
mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

