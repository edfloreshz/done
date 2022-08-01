use crate::data::plugins::local::LocalProvider;
use serde::{Deserialize, Serialize};
use crate::data::plugins::today::TodayProvider;

pub mod local;
pub mod today;

#[derive(Debug, Serialize, Deserialize)]
pub struct Plugins {
	pub(crate) local: LocalProvider,
	pub(crate) today: TodayProvider,
}

impl Plugins {
	pub fn init() -> Self {
		debug!("Initializing services...");
		let plugins = Self {
			local: Default::default(),
			today: Default::default()
		};
		debug!("Services initialized...");
		plugins
	}
}
