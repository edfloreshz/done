use std::pin::Pin;

use anyhow::{Context, Result};
use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use futures::Stream;
use url::Url;

use crate::{
	models::{list::List, task::Task},
	schema::lists::dsl::lists,
	schema::lists::*,
	schema::tasks::dsl::tasks,
	schema::tasks::*,
	task_service::TodoProvider,
};

use super::database::{
	models::{list::QueryableList, task::QueryableTask},
	Database,
};

#[derive(Debug, Clone)]
pub struct ComputerStorage {
	database: Database,
}

impl ComputerStorage {
	pub(crate) fn new(application_id: String) -> Self {
		let database =
			Database::new(application_id).expect("Failed to create database");

		Self { database }
	}
}

#[async_trait]
impl TodoProvider for ComputerStorage {
	async fn handle_uri_params(&mut self, _uri: Url) -> Result<()> {
		Ok(())
	}

	fn login(&self) -> Result<()> {
		Ok(())
	}

	fn logout(&self) -> Result<()> {
		Ok(())
	}

	fn available(&self) -> bool {
		true
	}

	fn stream_support(&self) -> bool {
		false
	}

	async fn read_tasks(&mut self) -> Result<Vec<Task>> {
		let task_list: Vec<Task> = tasks
			.load::<QueryableTask>(&mut self.database.establish_connection()?)?
			.iter()
			.map(|t| t.clone().into())
			.collect();

		Ok(task_list)
	}

	async fn get_tasks(
		&mut self,
		_parent_list: String,
	) -> Result<Pin<Box<dyn Stream<Item = Task> + Send>>> {
		todo!("This service does not implement streams")
	}

	async fn read_tasks_from_list(
		&mut self,
		parent_list: String,
	) -> Result<Vec<Task>> {
		let response: Vec<Task> = tasks
			.filter(parent.eq(parent_list))
			.load::<QueryableTask>(&mut self.database.establish_connection()?)?
			.iter()
			.map(|t| t.clone().into())
			.collect();

		Ok(response)
	}

	async fn read_task(
		&mut self,
		_task_list_id: String,
		task_id: String,
	) -> Result<Task> {
		let task: QueryableTask = tasks
			.find(task_id)
			.first(&mut self.database.establish_connection()?)
			.context("Failed to fetch list of tasks.")?;

		Ok(task.into())
	}

	async fn create_task(&mut self, task: Task) -> Result<()> {
		let queryable_task: QueryableTask = task.into();

		diesel::insert_into(tasks)
			.values(&queryable_task)
			.execute(&mut self.database.establish_connection()?)?;

		Ok(())
	}

	async fn update_task(&mut self, task: Task) -> Result<Task> {
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
			.execute(&mut self.database.establish_connection()?)
			.context("Failed to update task.")?;

		Ok(original_task)
	}

	async fn delete_task(
		&mut self,
		_list_id: String,
		task_id: String,
	) -> Result<()> {
		diesel::delete(tasks.filter(id_task.eq(task_id)))
			.execute(&mut self.database.establish_connection()?)?;

		Ok(())
	}

	async fn read_lists(&mut self) -> Result<Vec<List>> {
		let results = lists
			.load::<QueryableList>(&mut self.database.establish_connection()?)?;

		let results: Vec<List> = results.iter().map(|t| t.clone().into()).collect();
		Ok(results)
	}

	async fn get_lists(
		&mut self,
	) -> Result<Pin<Box<dyn Stream<Item = List> + Send>>> {
		todo!("This service does not implement streams")
	}

	async fn read_list(&mut self, id: String) -> Result<List> {
		let result: QueryableList = lists
			.find(id)
			.first(&mut self.database.establish_connection()?)?;
		Ok(result.into())
	}

	async fn create_list(&mut self, list: List) -> Result<List> {
		let list: QueryableList = list.into();

		diesel::insert_into(lists)
			.values(&list)
			.execute(&mut self.database.establish_connection()?)?;

		Ok(list.into())
	}

	async fn update_list(&mut self, list: List) -> Result<()> {
		let list: QueryableList = list.into();

		diesel::update(lists.filter(id_list.eq(list.id_list.clone())))
			.set((name.eq(list.name.clone()), icon_name.eq(list.icon_name)))
			.execute(&mut self.database.establish_connection()?)
			.context("Failed to update list.")?;

		Ok(())
	}

	async fn delete_list(&mut self, id: String) -> Result<()> {
		diesel::delete(lists.filter(id_list.eq(id)))
			.execute(&mut self.database.establish_connection()?)?;
		Ok(())
	}
}
