use crate::{
	services::{
		local::service::ComputerStorage, microsoft::service::MicrosoftService,
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
	Computer,
	Microsoft,
}

impl Service {
	pub fn icon(&self) -> &str {
		match self {
			Service::Smart => "dialog-information-symbolic",
			Service::Computer => {
				"/dev/edfloreshz/Done/icons/scalable/services/computer.png"
			},
			Service::Microsoft => {
				"/dev/edfloreshz/Done/icons/scalable/services/microsoft-todo.png"
			},
		}
	}
}

impl ToString for Service {
	fn to_string(&self) -> String {
		match self {
			Service::Smart => "Smart lists".into(),
			Service::Computer => "Computer".into(),
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
			Service::Computer => Box::new(ComputerStorage::new()),
			Service::Microsoft => Box::new(MicrosoftService::new()),
		}
	}

	/// Convenience method to get the list of services.
	pub fn list() -> Vec<Self> {
		Self::iter().collect()
	}
}
