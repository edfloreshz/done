#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use crate::provider::{List, ProviderRequest, Task};
use chrono::Utc;
use diesel_migrations::EmbeddedMigrations;
use uuid::Uuid;

mod data;
mod models;
pub mod plugins;
pub mod provider;
mod schema;

pub use tonic::transport::Channel;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

impl List {
	pub fn new(name: &str, icon: &str, provider: &str) -> Self {
		let icon = if icon.is_empty() {
			None
		} else {
			Some(icon.to_string())
		};
		Self {
			id: Uuid::new_v4().to_string(),
			name: name.to_string(),
			is_owner: true,
			icon,
			provider: provider.to_string(),
		}
	}
}

impl Task {
	pub fn new(title: String, parent: String) -> Self {
		Self {
			id: Uuid::new_v4().to_string(),
			parent,
			title,
			body: None,
			importance: 0,
			favorite: false,
			is_reminder_on: false,
			due_date: None,
			reminder_date: None,
			completed_on: None,
			status: 0,
			created_date_time: Utc::now().timestamp(),
			last_modified_date_time: Utc::now().timestamp(),
		}
	}
}

impl ProviderRequest {
	pub fn new(list: Option<List>, task: Option<Task>) -> Self {
		Self { list, task }
	}
}
