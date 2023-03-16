use std::collections::HashMap;

use done_provider::Task;

use crate::application::plugin::Plugin;

pub struct AllModel {
	pub tasks: HashMap<Plugin, Vec<Task>>,
}
