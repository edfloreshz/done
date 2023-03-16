use std::collections::HashMap;

use done_provider::Task;

use crate::application::plugin::Plugin;

pub struct Next7DaysModel {
	pub tasks: HashMap<Plugin, Vec<Task>>,
}
