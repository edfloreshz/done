use std::collections::HashMap;

use proto_rust::Task;

use crate::application::plugin::Plugin;

pub struct TodayModel {
	pub tasks: HashMap<Plugin, Vec<Task>>,
}
