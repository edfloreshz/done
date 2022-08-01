use anyhow::Context;
use diesel::prelude::*;
use diesel::{Connection, SqliteConnection};
use serde::{Deserialize, Serialize};

use crate::data::models::generic::lists::GenericList;
use crate::data::models::generic::tasks::GenericTask;
use crate::data::models::queryable::list::QueryableList;
use crate::data::models::queryable::task::QueryableTask;

use crate::data::traits::provider::{Provider, ProviderType};
use crate::embedded_migrations;
use crate::gtk::Image;

pub mod models;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodayProvider {
	id: String,
	name: String,
	provider_type: ProviderType,
	description: String,
	enabled: bool,
	smart: bool,
	icon: String,
}

impl Default for TodayProvider {
	fn default() -> Self {
		Self {
			id: "today".to_string(),
			name: "Today".to_string(),
			provider_type: ProviderType::Local,
			description: "Today list".to_string(),
			enabled: true,
			smart: true,
			icon: "org.gnome.Calendar.Devel-symbolic".to_string(),
		}
	}
}

impl Provider for TodayProvider {
	fn get_id(&self) -> &str {
		&self.id
	}

	fn get_name(&self) -> &str {
		&self.name
	}

	fn get_provider_type(&self) -> ProviderType {
		self.provider_type
	}

	fn get_description(&self) -> &str {
		&self.description
	}

	fn is_enabled(&self) -> bool {
		self.enabled
	}

	fn is_smart(&self) -> bool {
		self.smart
	}

	fn get_icon_name(&self) -> &str {
		&self.icon
	}

	fn get_icon(&self) -> Image {
		Image::from_icon_name(&self.icon)
	}

	fn set_enabled(&mut self) {
		self.enabled = true;
	}

	fn set_disabled(&mut self) {
		self.enabled = false;
	}

	fn new() -> Self
	where
		Self: Sized,
	{
		Self::default()
	}

	fn refresh(&self) -> anyhow::Result<()> {
		todo!()
	}

	fn read_tasks_from_list(&self, id: &str) -> anyhow::Result<Vec<GenericTask>> {
		use crate::schema::tasks::dsl::*;

		let results = tasks
			.filter(id_list.eq(id))
			.load::<QueryableTask>(&establish_connection()?)?;
		let results: Vec<GenericTask> =
			results.iter().map(|task| task.to_owned().into()).collect();
		Ok(results)
	}

	fn read_task(&self, _id: &str) -> anyhow::Result<GenericTask> {
		todo!()
	}

	fn create_task(
		&self,
		list: &GenericList,
		task: GenericTask,
	) -> anyhow::Result<GenericTask> {
		use crate::schema::tasks::dsl::*;

		let task: GenericTask = task.into();
		let list: GenericList = list.to_owned().into();

		let inserted_task = QueryableTask::new(task.title, list.id_list);
		diesel::insert_into(tasks)
			.values(&inserted_task)
			.execute(&establish_connection()?)?;
		Ok(inserted_task.into())
	}

	fn update_task(&self, task: GenericTask) -> anyhow::Result<()> {
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
			.execute(&establish_connection()?)?;
		Ok(())
	}

	fn remove_task(&self, task_id: &str) -> anyhow::Result<()> {
		use crate::schema::tasks::dsl::*;
		diesel::delete(tasks.filter(id_task.eq(task_id)))
			.execute(&establish_connection()?)?;
		Ok(())
	}

	fn read_task_lists(&self) -> anyhow::Result<Vec<GenericList>> {
		use crate::schema::lists::dsl::*;

		let results = lists
			.filter(provider.eq(self.get_id()))
			.load::<QueryableList>(&establish_connection()?)?;
		let results: Vec<GenericList> =
			results.into_iter().map(|ql| ql.into()).collect();
		Ok(results)
	}

	fn create_task_list(
		&mut self,
		list_provider: &str,
		name: &str,
		icon: &str,
	) -> anyhow::Result<GenericList> {
		use crate::schema::lists::dsl::*;

		let new_list =
			QueryableList::new(name, Some(icon.into()), list_provider.into());
		diesel::insert_into(lists)
			.values(&new_list)
			.execute(&establish_connection()?)?;
		let list: GenericList = new_list.into();
		Ok(list)
	}

	fn update_task_list(
		&self,
		list: GenericList,
		name: &str,
	) -> anyhow::Result<()> {
		use crate::schema::lists::dsl::*;

		let queryable_list = QueryableList {
			id_list: list.id_list.clone(),
			display_name: name.to_string(),
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
			.execute(&establish_connection()?)?;
		Ok(())
	}

	fn remove_task_list(&self, list: GenericList) -> anyhow::Result<()> {
		use crate::schema::lists::dsl::*;
		diesel::delete(lists.filter(id_list.eq(list.clone().id_list)))
			.execute(&establish_connection()?)?;
		Ok(())
	}
}

fn establish_connection() -> anyhow::Result<SqliteConnection> {
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
