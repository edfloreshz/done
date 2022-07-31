use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::lists;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable)]
#[table_name = "lists"]
pub struct QueryableList {
	pub id_list: String,
	pub display_name: String,
	pub is_owner: bool,
	pub count: i32,
	pub icon_name: Option<String>,
	pub(crate) provider: String,
}

impl QueryableList {
	pub fn new(
		display_name: &str,
		icon_name: Option<String>,
		list_provider: String,
	) -> Self {
		Self {
			id_list: Uuid::new_v4().to_string(),
			display_name: display_name.to_string(),
			is_owner: true,
			count: 0,
			icon_name,
			provider: list_provider,
		}
	}
}
