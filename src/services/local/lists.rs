use anyhow::Result;
use diesel::prelude::*;

use crate::models::list::List;
use crate::storage::database::DatabaseConnection;
use crate::schema::lists::dsl::*;

pub fn get_lists() -> Result<Vec<List>> {
    let connection = DatabaseConnection::establish_connection();
    let results = lists.load::<List>(&connection)?;
    Ok(results)
}

pub fn post_list(name: String) -> Result<List> {
    let connection = DatabaseConnection::establish_connection();
    let new_list = List::new(&*name, "view-list-symbolic");
    diesel::insert_into(lists)
        .values(&new_list)
        .execute(&connection)?;
    Ok(new_list)
}

pub fn patch_list(list: &List) -> Result<()> {
    let connection = DatabaseConnection::establish_connection();
    let list = list.to_owned();
    diesel::update(lists
        .filter(id_list.eq(list.id_list)))
        .set((
            display_name.eq(list.display_name),
            is_owner.eq(list.is_owner),
            count.eq(list.count),
            icon_name.eq(list.icon_name)
        ))
        .execute(&connection)?;
    Ok(())
}
