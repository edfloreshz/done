use std::collections::HashMap;

use proto_rust::Task;

use crate::application::plugin::Plugin;

pub struct Next7DaysModel {
	pub tasks: HashMap<Plugin, Vec<Task>>,
}
