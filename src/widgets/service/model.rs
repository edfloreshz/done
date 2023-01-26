use crate::application::plugin::Plugin;

#[derive(Debug, Default)]
pub struct ServiceModel {
	pub plugin: Plugin,
	pub enabled: bool,
	pub installed: bool,
	pub update: bool,
	pub first_load: bool,
}

#[derive(Debug)]
pub enum UpdateStatus {
	Completed,
	Failed,
}
