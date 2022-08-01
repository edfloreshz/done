use crate::data::plugins::local::LocalProvider;
use serde::{Deserialize, Serialize};

pub mod local;

#[derive(Debug, Serialize, Deserialize)]
pub struct Plugins {
	pub(crate) local: LocalProvider,
}

impl Plugins {
	pub fn init() -> Self {
		debug!("Initializing services...");
		let plugins = Self {
			local: Default::default(),
		};
		debug!("Services initialized...");
		plugins
	}
}
