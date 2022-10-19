use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::lists;
use crate::traits::List;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = lists)]
pub struct QueryableList {
	pub id_list: String,
	pub display_name: String,
	pub is_owner: bool,
	pub count: i32,
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
			display_name: display_name.to_string(),
			is_owner: true,
			count: 0,
			icon_name,
			provider: list_provider,
		}
	}

	pub fn from(list: impl List) -> Self {
		Self {
			id_list: list.id(),
			display_name: list.display_name(),
			is_owner: list.is_owner(),
			count: list.count(),
			icon_name: list.icon_name(),
			provider: list.provider()
		}
	}
}

impl List for QueryableList {
    fn id(&self) -> String {
        self.id_list.clone()
    }

    fn display_name(&self) -> String {
        self.display_name.clone()
    }

    fn is_owner(&self) -> bool {
        self.is_owner
    }

    fn count(&self) -> i32 {
        self.count
    }

    fn icon_name(&self) -> Option<String> {
        self.icon_name.clone()
    }

    fn provider(&self) -> String {
        self.provider.clone()
    }

    fn is_smart(&self) -> bool {
        false
    }
}