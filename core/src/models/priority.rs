use serde::{Deserialize, Serialize};

#[derive(
	Clone,
	Copy,
	Default,
	Debug,
	PartialEq,
	Eq,
	Hash,
	PartialOrd,
	Ord,
	Serialize,
	Deserialize,
)]
pub enum Priority {
	#[default]
	Low = 0,
	Normal = 1,
	High = 2,
}

impl From<i32> for Priority {
	fn from(value: i32) -> Self {
		match value {
			0 => Priority::Low,
			1 => Priority::Normal,
			2 => Priority::High,
			_ => panic!("Invalid value for Priority"),
		}
	}
}

impl Into<i32> for Priority {
	fn into(self) -> i32 {
		match self {
			Priority::Low => 0,
			Priority::Normal => 1,
			Priority::High => 2,
		}
	}
}

impl Priority {
	pub fn as_str_name(&self) -> &'static str {
		match self {
			Priority::Low => "LOW",
			Priority::Normal => "NORMAL",
			Priority::High => "HIGH",
		}
	}

	pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
		match value {
			"LOW" => Some(Self::Low),
			"NORMAL" => Some(Self::Normal),
			"HIGH" => Some(Self::High),
			_ => None,
		}
	}
}

impl From<msft_todo_types::importance::Importance> for Priority {
	fn from(value: msft_todo_types::importance::Importance) -> Self {
		match value {
			msft_todo_types::importance::Importance::Low => Self::Low,
			msft_todo_types::importance::Importance::Normal => Self::Normal,
			msft_todo_types::importance::Importance::High => Self::High,
		}
	}
}
