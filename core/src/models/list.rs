use msft_todo_types::list::ToDoTaskList;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::service::Service;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct List {
	pub id: String,
	pub name: String,
	pub description: String,
	pub icon: Option<String>,
	pub service: Service,
}

impl List {
	pub fn new(name: &str, service: Service) -> Self {
		Self {
			id: Uuid::new_v4().to_string(),
			name: name.to_string(),
			service,
			description: String::new(),
			icon: Some("✍️".to_string()),
		}
	}
}

impl From<ToDoTaskList> for List {
	fn from(task: ToDoTaskList) -> Self {
		Self {
			id: task.id,
			name: task.display_name,
			description: String::new(),
			icon: Some("✍️".to_string()),
			service: Service::Microsoft,
		}
	}
}
