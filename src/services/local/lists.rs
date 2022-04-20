use anyhow::Result;
use diesel::prelude::*;

use crate::models::list::List;
use crate::storage::database::DatabaseConnection;

pub fn get_lists() -> Result<Vec<List>> {
    use crate::schema::lists::dsl::*;
    let connection = DatabaseConnection::establish_connection();
    let results = lists.load::<List>(&connection)?;
    Ok(results)
}

pub fn post_list(name: String) -> Result<List> {
    use crate::schema::lists::dsl::*;

    let connection = DatabaseConnection::establish_connection();
    let new_list = List::new(&*name, "");
    diesel::insert_into(lists)
        .values(&new_list)
        .execute(&connection)?;
    Ok(new_list)
}