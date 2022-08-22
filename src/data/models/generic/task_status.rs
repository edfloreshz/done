use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
	NotStarted,
	Completed,
}

impl Default for TaskStatus {
	fn default() -> Self {
		TaskStatus::NotStarted
	}
}

impl Display for TaskStatus {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			TaskStatus::NotStarted => write!(f, "notStarted"),
			TaskStatus::Completed => write!(f, "completed"),
		}
	}
}

impl FromStr for TaskStatus {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"notstarted" => Ok(TaskStatus::NotStarted),
			"completed" => Ok(TaskStatus::Completed),
			_ => Err(()),
		}
	}
}

impl TaskStatus {
	pub fn as_bool(&self) -> bool {
		matches!(self, Self::Completed)
	}
}
