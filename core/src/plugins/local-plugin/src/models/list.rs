use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use done_core::provider::List;

use crate::schema::lists;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = lists)]
pub struct QueryableList {
	pub id_list: String,
	pub name: String,
	pub is_owner: bool,
	pub icon_name: Option<String>,
	pub provider: String,
}

impl QueryableList {
	pub fn new(
		display_name: &str,
		icon_name: Option<String>,
		list_provider: String,
	) -> Self {
		Self {
			id_list: Uuid::new_v4().to_string(),
			name: display_name.to_string(),
			is_owner: true,
			icon_name,
			provider: list_provider,
		}
	}
}

impl Into<List> for QueryableList {
	fn into(self) -> List {
		List {
			id: self.id_list,
			name: self.name,
			is_owner: self.is_owner,
			icon: self.icon_name,
			provider: self.provider
		}
	}
}

impl From<List> for QueryableList {
	fn from(task: List) -> Self {
		Self {
			id_list: task.id,
			name: task.name,
			is_owner: task.is_owner,
			icon_name: task.icon,
			provider: task.provider
		}
	}
}
