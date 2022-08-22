use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskImportance {
	Low,
	Normal,
	High,
}

impl Display for TaskImportance {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			TaskImportance::Low => write!(f, "low"),
			TaskImportance::Normal => write!(f, "normal"),
			TaskImportance::High => write!(f, "high"),
		}
	}
}

impl FromStr for TaskImportance {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"low" => Ok(TaskImportance::Low),
			"normal" => Ok(TaskImportance::Normal),
			"high" => Ok(TaskImportance::High),
			_ => Err(()),
		}
	}
}

impl Default for TaskImportance {
	fn default() -> Self {
		TaskImportance::Normal
	}
}
