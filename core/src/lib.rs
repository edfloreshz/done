pub mod database;
pub mod models;
pub mod schema;
pub mod setup;

use anyhow::{Context, Result};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use models::{List, QueryableList};

use crate::{
	database::{task::QueryableTask, Database},
	models::Task,
	schema::lists::dsl::lists,
	schema::lists::*,
	schema::tasks::dsl::tasks,
	schema::tasks::*,
};

#[derive(Debug, Clone, Copy)]
pub struct LocalStorage;

impl LocalStorage {
	pub fn new() -> Self {
		Self
	}

	pub async fn get_all_tasks(&self) -> Result<Vec<Task>> {
		let task: Vec<Task> = tasks
			.load::<QueryableTask>(&mut Database::establish_connection()?)?
			.iter()
			.map(|t| t.clone().into())
			.collect();

		Ok(task.into())
	}

	pub async fn get_task(&self, id: String) -> Result<Task> {
		let task: QueryableTask = tasks
			.find(id)
			.first(&mut Database::establish_connection()?)
			.context("Failed to fetch list of tasks.")?;

		Ok(task.into())
	}

	pub async fn get_tasks(&self, id: String) -> Result<Vec<Task>> {
		let response: Vec<Task> = tasks
			.filter(parent.eq(id))
			.load::<QueryableTask>(&mut Database::establish_connection()?)?
			.iter()
			.map(|t| t.clone().into())
			.collect();

		Ok(response)
	}

	pub async fn create_task(&self, task: Task) -> Result<()> {
		let queryable_task: QueryableTask = task.clone().into();

		diesel::insert_into(tasks)
			.values(&queryable_task)
			.execute(&mut Database::establish_connection()?)?;

		Ok(())
	}

	pub async fn update_task(&self, task: Task) -> Result<Task> {
		let original_task = task.clone();
		let queryable_task: QueryableTask = task.into();

		diesel::update(tasks.filter(id_task.eq(queryable_task.id_task.clone())))
			.set((
				id_task.eq(queryable_task.id_task),
				parent.eq(queryable_task.parent),
				title.eq(queryable_task.title),
				favorite.eq(queryable_task.favorite),
				today.eq(queryable_task.today),
				status.eq(queryable_task.status),
				priority.eq(queryable_task.priority),
				sub_tasks.eq(queryable_task.sub_tasks),
				tags.eq(queryable_task.tags),
				notes.eq(queryable_task.notes),
				completion_date.eq(queryable_task.completion_date),
				deletion_date.eq(queryable_task.deletion_date),
				due_date.eq(queryable_task.due_date),
				reminder_date.eq(queryable_task.reminder_date),
				recurrence.eq(queryable_task.recurrence),
				created_date_time.eq(queryable_task.created_date_time),
				last_modified_date_time.eq(queryable_task.last_modified_date_time),
			))
			.execute(&mut Database::establish_connection()?)
			.context("Failed to update task.")?;

		Ok(original_task)
	}

	pub async fn delete_task(&self, id: String) -> Result<()> {
		diesel::delete(tasks.filter(id_task.eq(id)))
			.execute(&mut Database::establish_connection()?)?;

		Ok(())
	}

	pub async fn get_list(&self, id: String) -> Result<List> {
		let result: QueryableList = lists
			.find(id)
			.first(&mut Database::establish_connection()?)?;
		Ok(result.into())
	}

	pub async fn get_lists(&self) -> Result<Vec<List>> {
		let results =
			lists.load::<QueryableList>(&mut Database::establish_connection()?)?;

		let results: Vec<List> = results.iter().map(|t| t.clone().into()).collect();
		Ok(results)
	}

	pub async fn get_list_ids(&self) -> Result<Vec<String>> {
		let result: Vec<String> = lists
			.select(id_list)
			.load::<String>(&mut Database::establish_connection()?)
			.context("Failed to fetch list of tasks.")?;
		Ok(result)
	}

	pub async fn create_list(&self, list: List) -> Result<List> {
		let list: QueryableList = list.into();

		diesel::insert_into(lists)
			.values(&list)
			.execute(&mut Database::establish_connection()?)?;

		Ok(list.into())
	}

	pub async fn update_list(&self, list: List) -> Result<()> {
		let list: QueryableList = list.into();

		diesel::update(lists.filter(id_list.eq(list.id_list.clone())))
			.set((name.eq(list.name.clone()), icon_name.eq(list.icon_name)))
			.execute(&mut Database::establish_connection()?)
			.context("Failed to update list.")?;

		Ok(())
	}

	pub async fn delete_list(&self, id: String) -> Result<()> {
		diesel::delete(lists.filter(id_list.eq(id)))
			.execute(&mut Database::establish_connection()?)?;
		Ok(())
	}

	pub async fn get_tasks_from_list(&self, id: String) -> Result<Vec<Task>> {
		let result: Vec<QueryableTask> = tasks
			.filter(parent.eq(id))
			.load::<QueryableTask>(&mut Database::establish_connection()?)
			.context("Failed to fetch list of tasks.")?;
		let results: Vec<Task> = result.iter().map(|t| t.clone().into()).collect();
		Ok(results)
	}

	pub async fn get_task_ids_from_list(
		&self,
		id: String,
	) -> Result<Vec<String>> {
		let result: Vec<String> = tasks
			.select(id_task)
			.filter(parent.eq(id))
			.load::<String>(&mut Database::establish_connection()?)
			.context("Failed to fetch list of tasks.")?;
		Ok(result)
	}

	pub async fn get_task_count_from_list(&self, id: String) -> Result<i64> {
		let count: i64 = tasks
			.filter(id_task.eq(id))
			.count()
			.get_result(&mut Database::establish_connection()?)?;
		Ok(count)
	}
}
