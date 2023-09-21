use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{models::list::List, schema::lists, service::Service};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = lists)]
pub struct QueryableList {
	pub id_list: String,
	pub name: String,
	pub description: String,
	pub icon_name: Option<String>,
}

impl QueryableList {
	pub fn new(
		display_name: &str,
		description: &str,
		icon_name: Option<String>,
	) -> Self {
		Self {
			id_list: Uuid::new_v4().to_string(),
			name: display_name.to_string(),
			description: description.to_string(),
			icon_name,
		}
	}
}

impl From<QueryableList> for List {
	fn from(value: QueryableList) -> Self {
		List {
			id: value.id_list,
			name: value.name,
			service: Service::Computer,
			icon: value.icon_name,
			description: value.description,
		}
	}
}

impl From<List> for QueryableList {
	fn from(list: List) -> Self {
		Self {
			id_list: list.id,
			name: list.name,
			description: list.description,
			icon_name: list.icon,
		}
	}
}
