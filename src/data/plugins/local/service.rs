use std::collections::VecDeque;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};

use crate::data::models::generic::lists::GenericList;
use crate::data::models::generic::tasks::GenericTask;
use crate::data::models::queryable::list::QueryableList;
use crate::data::models::queryable::task::QueryableTask;
use crate::data::plugins::local::models::lists::LocalList;
use crate::data::plugins::local::models::tasks::LocalTask;
use crate::data::plugins::local::LocalProvider;
use crate::data::traits::provider::{ProviderService, TaskProvider};
use crate::embedded_migrations;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LocalService {
	pub provider: LocalProvider,
	pub lists: VecDeque<LocalList>,
	pub tasks: VecDeque<LocalTask>,
}

impl ProviderService for LocalService {
	fn init() -> Self {
		let mut local = Self {
			provider: Default::default(),
			lists: Default::default(),
			tasks: Default::default(),
		};
		local.refresh_lists().unwrap();
		local.refresh_tasks().unwrap();
		local
	}

	fn establish_connection(&self) -> Result<SqliteConnection> {
		let database_path = dirs::data_dir()
			.with_context(|| "Failed to get data directory.")?
			.join("done/dev.edfloreshz.Done.db");
		let database_url = database_path
			.to_str()
			.with_context(|| "Failed to convert path to string")?;
		let connection = SqliteConnection::establish(database_url)
			.with_context(|| "Error connecting to database")?;
		embedded_migrations::run(&connection)?;

		Ok(connection)
	}

	fn refresh_tasks(&mut self) -> Result<()> {
		for list in &self.lists {
			for task in self.read_tasks_from_list(&*list.id_list)? {
				self.tasks.push_back(task.into())
			}
		}
		Ok(())
	}

	fn refresh_lists(&mut self) -> Result<()> {
		self.lists = self
			.read_task_lists()?
			.iter()
			.map(|list| list.to_owned().into())
			.collect();
		Ok(())
	}

	fn get_provider(&self) -> Box<dyn TaskProvider> {
		Box::new(self.clone().provider)
	}

	fn get_tasks(&self) -> Vec<GenericTask> {
		self
			.tasks
			.iter()
			.map(|task| task.to_owned().into())
			.collect()
	}

	fn get_task_lists(&self) -> Vec<GenericList> {
		self
			.lists
			.iter()
			.map(|list| list.to_owned().into())
			.collect()
	}

	fn read_tasks_from_list(&self, id: &str) -> anyhow::Result<Vec<GenericTask>> {
		use crate::schema::tasks::dsl::*;

		let results = tasks
			.filter(id_list.eq(id))
			.load::<QueryableTask>(&self.establish_connection()?)?;
		let results: Vec<GenericTask> =
			results.iter().map(|task| task.to_owned().into()).collect();
		Ok(results)
	}

	fn read_task(&self, id: &str) -> anyhow::Result<GenericTask> {
		todo!()
	}

	fn create_task(
		&self,
		list: GenericList,
		task: GenericTask,
	) -> Result<GenericTask> {
		use crate::schema::tasks::dsl::*;

		let task: GenericTask = task.into();
		let list: GenericList = list.into();

		let inserted_task = QueryableTask::new(task.title, list.id_list);
		diesel::insert_into(tasks)
			.values(&inserted_task)
			.execute(&self.establish_connection()?)?;
		Ok(inserted_task.into())
	}

	fn update_task(&self, task: GenericTask) -> Result<()> {
		use crate::schema::tasks::dsl::*;

		let task: QueryableTask = task.into();
		diesel::update(tasks.filter(id_task.eq(task.id_task)))
			.set((
				id_list.eq(task.id_list),
				title.eq(task.title),
				body.eq(task.body),
				completed_on.eq(task.completed_on),
				due_date.eq(task.due_date),
				importance.eq(task.importance),
				favorite.eq(task.favorite),
				is_reminder_on.eq(task.is_reminder_on),
				reminder_date.eq(task.reminder_date),
				status.eq(task.status),
				created_date_time.eq(task.created_date_time),
				last_modified_date_time.eq(task.last_modified_date_time),
			))
			.execute(&self.establish_connection()?)?;
		Ok(())
	}

	fn remove_task(&self, task_id: &str) -> anyhow::Result<()> {
		use crate::schema::tasks::dsl::*;
		diesel::delete(tasks.filter(id_task.eq(task_id)))
			.execute(&self.establish_connection()?)?;
		Ok(())
	}

	fn read_task_lists(&self) -> anyhow::Result<Vec<GenericList>> {
		use crate::schema::lists::dsl::*;

		let results = lists
			.filter(provider.eq(self.provider.get_id()))
			.load::<QueryableList>(&self.establish_connection()?)?;
		let results: Vec<GenericList> =
			results.into_iter().map(|ql| ql.into()).collect();
		Ok(results)
	}

	fn create_task_list(&self, list: GenericList) -> Result<GenericList> {
		use crate::schema::lists::dsl::*;

		let new_list = QueryableList::new(
			&*list.display_name,
			Some("list-compact-symbolic".into()),
			list.provider.clone(),
		);
		diesel::insert_into(lists)
			.values(&new_list)
			.execute(&self.establish_connection()?)?;
		Ok(new_list.into())
	}

	fn update_task_list(&self, list: GenericList) -> Result<()> {
		use crate::schema::lists::dsl::*;

		let queryable_list = QueryableList {
			id_list: list.id_list.clone(),
			display_name: list.display_name.clone(),
			is_owner: list.is_owner,
			count: list.count,
			icon_name: list.icon_name.clone(),
			provider: list.provider.clone(),
		};
		diesel::update(lists.filter(id_list.eq(queryable_list.id_list.clone())))
			.set((
				display_name.eq(queryable_list.display_name.clone()),
				is_owner.eq(queryable_list.is_owner),
				count.eq(queryable_list.count),
				icon_name.eq(queryable_list.icon_name),
			))
			.execute(&self.establish_connection()?)?;
		Ok(())
	}

	fn remove_task_list(&self, list: GenericList) -> Result<()> {
		use crate::schema::lists::dsl::*;
		diesel::delete(lists.filter(id_list.eq(list.id_list)))
			.execute(&self.establish_connection()?)?;
		Ok(())
	}
}
