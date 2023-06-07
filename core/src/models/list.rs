use msft_todo_types::list::ToDoTaskList;
use regex::Regex;
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
		let display_name = remove_emoji(&task.display_name);
		let icon = extract_emoji(&task.display_name);
		Self {
			id: task.id,
			name: display_name,
			description: String::new(),
			icon,
			service: Service::Microsoft,
		}
	}
}

fn extract_emoji(string: &str) -> Option<String> {
	let re = Regex::new(r"\p{Emoji}").unwrap();
	let match_result = re.find(&string);

	match match_result {
		Some(matched) => Some(matched.as_str().to_string()),
		None => None,
	}
}

fn remove_emoji(string: &str) -> String {
	let re = Regex::new(r"([\p{Emoji}\u{FE0E}\u{FE0F}])").unwrap();
	re.replace_all(string, "").trim().to_string()
}
