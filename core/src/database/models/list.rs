use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::lists;

#[derive(Clone, PartialEq)]
pub struct List {
	pub id: String,
	pub name: String,
	pub icon: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = lists)]
pub struct QueryableList {
	pub id_list: String,
	pub name: String,
	pub icon_name: Option<String>,
}

impl QueryableList {
	pub fn new(display_name: &str, icon_name: Option<String>) -> Self {
		Self {
			id_list: Uuid::new_v4().to_string(),
			name: display_name.to_string(),
			icon_name,
		}
	}
}

impl From<QueryableList> for List {
	fn from(value: QueryableList) -> Self {
		List {
			id: value.id_list,
			name: value.name,
			icon: value.icon_name,
		}
	}
}

impl From<List> for QueryableList {
	fn from(task: List) -> Self {
		Self {
			id_list: task.id,
			name: task.name,
			icon_name: task.icon,
		}
	}
}
