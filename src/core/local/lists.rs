use anyhow::Result;
use diesel::prelude::*;
use crate::models::list::List;
use crate::models::queryable::list::QueryableList;

use crate::schema::lists::dsl::*;
use crate::storage::DatabaseConnection;

pub fn get_lists() -> Result<Vec<List>> {
    let connection = DatabaseConnection::establish_connection();
    let results = lists.load::<QueryableList>(&connection)?;
    let results: Vec<List> = results
        .into_iter()
        .map(|ql| ql.into())
        .collect();
    Ok(results)
}

pub fn post_list(name: String) -> Result<List> {
    let connection = DatabaseConnection::establish_connection();
    let new_list = QueryableList::new(&*name, Some("view-list-symbolic".into()));
    diesel::insert_into(lists)
        .values(&new_list)
        .execute(&connection)?;
    Ok(new_list.into())
}

pub fn _patch_list(list: &List) -> Result<()> {
    let connection = DatabaseConnection::establish_connection();
    let list = QueryableList {
        id_list: list.id_list.clone(),
        display_name: list.display_name.clone(),
        is_owner: list.is_owner,
        count: list.count,
        icon_name: list.icon_name.clone(),
    };
    diesel::update(lists.filter(id_list.eq(list.id_list.clone())))
        .set((
            display_name.eq(list.display_name.clone()),
            is_owner.eq(list.is_owner),
            count.eq(list.count),
            icon_name.eq(list.icon_name.clone()),
        ))
        .execute(&connection)?;
    Ok(())
}
