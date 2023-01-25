use std::collections::HashMap;

use proto_rust::Task;

use crate::application::plugin::Plugin;

pub struct AllModel {
	pub tasks: HashMap<Plugin, Vec<Task>>,
}
