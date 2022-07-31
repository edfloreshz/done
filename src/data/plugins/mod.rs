use serde::{Deserialize, Serialize};

use crate::data::plugins::local::service::LocalService;
use crate::data::traits::provider::{Provider, Service};

pub mod local;

#[derive(Debug, Serialize, Deserialize)]
pub struct Plugins {
	pub(crate) local: LocalService,
}

impl Plugins {
	pub fn init() -> Self {
		let mut plugins = Self {
			local: Default::default(),
		};
		debug!("Initializing services...");
		plugins.local = LocalService::init();
		match plugins.local.establish_connection() {
			Ok(_) => {
				plugins.local.provider.set_enabled();
			},
			Err(err) => debug!("Error: {}", err),
		}
		debug!("Services initialized...");
		plugins
	}
}
