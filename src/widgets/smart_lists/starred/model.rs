use std::collections::HashMap;

use done_provider::Task;

use crate::application::plugin::Plugin;

pub struct StarredModel {
	pub tasks: HashMap<Plugin, Vec<Task>>,
}
