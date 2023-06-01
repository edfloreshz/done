use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

use crate::task_service::TaskService;

use self::{local::LocalStorage, microsoft::Microsoft};

pub mod local;
pub mod microsoft;

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
			Service::Local => Box::new(LocalStorage::new()),
			Service::Microsoft => Box::new(Microsoft::new()),
		}
	}

	/// Convenience method to get the list of services.
	pub fn list() -> Vec<Self> {
		Self::iter().collect()
	}
}
