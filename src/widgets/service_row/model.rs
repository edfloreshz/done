use crate::application::plugin::Plugin;

#[derive(Debug, Default)]
pub struct ServiceRowModel {
	pub plugin: Plugin,
	pub enabled: bool,
	pub installed: bool,
	pub update: bool,
	pub first_load: bool,
	pub process_id: usize,
}

#[derive(Debug)]
pub enum UpdateStatus {
	Completed,
	Failed,
}
