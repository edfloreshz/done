use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::list::List;

use crate::schema::lists;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable)]
#[table_name = "lists"]
pub struct QueryableList {
    pub id_list: String,
    pub display_name: String,
    pub is_owner: bool,
    pub count: i32,
    pub icon_name: Option<String>,
}

impl QueryableList {
    pub fn new(display_name: &str, icon_name: Option<String>) -> Self {
        Self {
            id_list: Uuid::new_v4().to_string(),
            display_name: display_name.to_string(),
            is_owner: true,
            count: 0,
            icon_name,
        }
    }
}

impl From<List> for QueryableList {
    fn from(list: List) -> Self {
        Self {
            id_list: list.id_list,
            display_name: list.display_name,
            is_owner: list.is_owner,
            count: list.count,
            icon_name: list.icon_name,
        }
    }
}

impl From<&List> for QueryableList {
    fn from(list: &List) -> Self {
        Self {
            id_list: list.id_list.clone(),
            display_name: list.display_name.clone(),
            is_owner: list.is_owner,
            count: list.count,
            icon_name: list.icon_name.clone(),
        }
    }
}