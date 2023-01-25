use std::collections::HashMap;

use proto_rust::Task;

use crate::application::plugin::Plugin;

pub struct StarredModel {
	pub tasks: HashMap<Plugin, Vec<Task>>,
}
