use crate::data::plugins::local::LocalProvider;
use serde::{Deserialize, Serialize};
use crate::data::plugins::all::AllProvider;
use crate::data::plugins::next7days::Next7DaysProvider;
use crate::data::plugins::starred::StarredProvider;
use crate::data::plugins::today::TodayProvider;

pub mod local;
pub mod today;
pub mod next7days;
pub mod all;
pub mod starred;

#[derive(Debug, Serialize, Deserialize)]
pub struct Plugins {
	pub(crate) local: LocalProvider,
	pub(crate) all: AllProvider,
	pub(crate) today: TodayProvider,
	pub(crate) starred: StarredProvider,
	pub(crate) next: Next7DaysProvider,
}

impl Plugins {
	pub fn init() -> Self {
		debug!("Initializing services...");
		let plugins = Self {
			local: Default::default(),
			all: Default::default(),
			today: Default::default(),
			starred: Default::default(),
			next: Default::default()
		};
		debug!("Services initialized...");
		plugins
	}
}
