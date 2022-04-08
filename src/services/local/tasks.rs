use anyhow::Result;
use diesel::prelude::*;
use crate::storage::database::DatabaseConnection;
use crate::models::task::QueryableTask;

pub fn get_tasks(list_id: String) -> Result<Vec<QueryableTask>> {
    use crate::schema::tasks::dsl::*;

    let connection = DatabaseConnection::establish_connection();
    let results = tasks
        .filter(id_list.eq(list_id))
        .load::<QueryableTask>(&connection)?;
    Ok(results)
}

pub fn post_task(list_id: String, name: String) -> Result<()> {
    use crate::schema::tasks::dsl::*;

    let connection = DatabaseConnection::establish_connection();
    let new_task = QueryableTask::new(name, list_id);
    diesel::insert_into(tasks)
        .values(&new_task)
        .execute(&connection)?;
    Ok(())
}