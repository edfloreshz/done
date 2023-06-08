use crate::{
	services::{
		local::service::LocalStorage, microsoft::service::Microsoft, smart::Smart,
	},
	task_service::TaskService,
};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

use std::sync::OnceLock;

static SMART: OnceLock<Smart> = OnceLock::new();
static LOCAL: OnceLock<LocalStorage> = OnceLock::new();
static MICROSOFT: OnceLock<Microsoft> = OnceLock::new();

#[derive(
	Debug,
	Default,
	EnumIter,
	EnumString,
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
	Smart,
	#[default]
	Local,
	Microsoft,
}

impl ToString for Service {
	fn to_string(&self) -> String {
		match self {
			Service::Smart => "Smart lists".into(),
			Service::Local => "Local tasks".into(),
			Service::Microsoft => "Microsoft To Do".into(),
		}
	}
}

impl Service {
	/// Finds the requested service and returns it.
	/// After implemeting the Service trait in your service
	/// struct, register your service here.
	pub fn get_service(&self) -> Box<dyn TaskService> {
		match self {
			Service::Smart => Box::new(*SMART.get_or_init(Smart::new)),
			Service::Local => Box::new(*LOCAL.get_or_init(LocalStorage::new)),
			Service::Microsoft => {
				Box::new(MICROSOFT.get_or_init(Microsoft::new).clone())
			},
		}
	}

	/// Convenience method to get the list of services.
	pub fn list() -> Vec<Self> {
		Self::iter().collect()
	}
}
