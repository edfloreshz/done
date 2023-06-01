use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{schema::lists, services::Service};

#[derive(Debug, Default, Clone, PartialEq)]
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
			service: Service::Local,
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
