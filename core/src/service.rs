use crate::{
	services::{
		local::service::LocalStorage, microsoft::service::MicrosoftService,
		smart::Smart,
	},
	task_service::TodoProvider,
};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

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

impl Service {
	pub fn icon(&self) -> &str {
		match self {
			Service::Smart => "dialog-information-symbolic",
			Service::Local => "document-save-symbolic",
			Service::Microsoft => "tools-symbolic",
		}
	}
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
	pub fn get_service(&self) -> Box<dyn TodoProvider> {
		match self {
			Service::Smart => Box::new(Smart::new()),
			Service::Local => Box::new(LocalStorage::new()),
			Service::Microsoft => Box::new(MicrosoftService::new()),
		}
	}

	/// Convenience method to get the list of services.
	pub fn list() -> Vec<Self> {
		Self::iter().collect()
	}
}
