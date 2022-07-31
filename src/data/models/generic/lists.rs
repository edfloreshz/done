use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::data::models::queryable::list::QueryableList;
use crate::data::plugins::local::models::lists::LocalList;
use crate::schema::lists::provider;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct GenericList {
    pub id_list: String,
    pub display_name: String,
    pub is_owner: bool,
    pub count: i32,
    pub icon_name: Option<String>,
    pub provider: String,
    pub is_smart: bool,
}

impl GenericList {
    pub fn new(display_name: &str, icon_name: &str, count: i32, list_provider: &str) -> Self {
        let icon_name = if icon_name.is_empty() {
            None
        } else {
            Some(icon_name.to_string())
        };
        Self {
            id_list: Uuid::new_v4().to_string(),
            display_name: display_name.to_string(),
            is_owner: true,
            count,
            icon_name,
            provider: list_provider.to_string(),
            is_smart: false,
        }
    }
}

impl From<QueryableList> for GenericList {
    fn from(queryable_list: QueryableList) -> Self {
        Self {
            id_list: queryable_list.id_list,
            display_name: queryable_list.display_name,
            is_owner: queryable_list.is_owner,
            count: queryable_list.count,
            icon_name: queryable_list.icon_name,
            provider: queryable_list.provider,
            is_smart: false
        }
    }
}

impl From<LocalList> for GenericList {
    fn from(local_list: LocalList) -> Self {
        Self {
            id_list: local_list.id_list,
            display_name: local_list.display_name,
            is_owner: local_list.is_owner,
            count: local_list.count,
            icon_name: local_list.icon_name,
            provider: local_list.provider,
            is_smart: false
        }
    }
}