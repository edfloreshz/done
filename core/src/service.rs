use crate::{
	services::{local::service::LocalStorage, microsoft::service::Microsoft},
	task_service::TaskService,
};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

use std::sync::OnceLock;

static LOCAL: OnceLock<LocalStorage> = OnceLock::new();
static MICROSOFT: OnceLock<Microsoft> = OnceLock::new();

#[derive(
	Debug,
	Default,
	EnumIter,
	EnumString,
	Display,
	Clone,
	Copy,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Serialize,
	Deserialize,
)]
pub enum Service {
	#[default]
	Local,
	Microsoft,
}

impl Service {
	/// Finds the requested service and returns it.
	/// After implemeting the Service trait in your service
	/// struct, register your service here.
	pub fn get_service(&self) -> Box<dyn TaskService> {
		match self {
			Service::Local => {
				Box::new(LOCAL.get_or_init(|| LocalStorage::new()).clone())
			},
			Service::Microsoft => {
				Box::new(MICROSOFT.get_or_init(|| Microsoft::new()).clone())
			},
		}
	}

	/// Convenience method to get the list of services.
	pub fn list() -> Vec<Self> {
		Self::iter().collect()
	}
}
