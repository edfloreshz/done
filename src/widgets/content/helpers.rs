use anyhow::{anyhow, Context, Result};
use chrono::{NaiveDateTime, Utc};
use done_local_storage::models::Task;
use done_local_storage::LocalStorage;
use relm4::ComponentController;
use relm4::{prelude::DynamicIndex, AsyncComponentSender};

use crate::factories::task::model::TaskInit;
use crate::widgets::sidebar::model::SidebarList;
use crate::{
	factories::details::model::TaskDetailsFactoryInit,
	widgets::task_entry::messages::TaskEntryInput,
};

use super::{
	messages::{ContentInput, ContentOutput},
	model::ContentModel,
};

pub fn reveal_task_details(
	model: &mut ContentModel,
	index: Option<DynamicIndex>,
	task: Task,
) {
	model.show_task_details = true;
	let mut guard = model.task_details_factory.guard();
	if let Some(task_index) = index {
		model.selected_task = Some(task.clone());
		guard.clear();
		guard.push_back(TaskDetailsFactoryInit::new(task, Some(task_index)));
	} else {
		guard.clear();
		guard.push_back(TaskDetailsFactoryInit::new(task, None));
	}
}

pub fn hide_flap(
	model: &mut ContentModel,
	sender: AsyncComponentSender<ContentModel>,
) {
	model.show_task_details = false;
	if let Some(list) = model.parent_list.clone() {
		sender.input(ContentInput::SelectList(SidebarList::Custom(list)))
	}
}

pub async fn add_task(
	model: &mut ContentModel,
	sender: AsyncComponentSender<ContentModel>,
	task: &mut Task,
) -> Result<()> {
	task.parent = model
		.parent_list
		.as_ref()
		.ok_or_else(|| anyhow!("This task doesn't have a parent list."))?
		.id
		.clone();
	let local = LocalStorage::new();
	match local.create_task(task.clone()).await {
		Ok(_) => {
			model.task_factory.guard().push_back(TaskInit::new(
				task.clone(),
				model.parent_list.clone(),
				model.compact,
			));
			model.show_task_details = false;
			sender.input(ContentInput::HideFlap);
			sender
				.output(ContentOutput::Notify("Task added successfully".into(), 1))
				.unwrap();
		},
		Err(_) => {
			sender
				.output(ContentOutput::Notify("Error adding task".into(), 2))
				.unwrap();
		},
	}

	Ok(())
}

pub async fn remove_task(
	model: &mut ContentModel,
	sender: AsyncComponentSender<ContentModel>,
	index: DynamicIndex,
) -> Result<()> {
	let mut guard = model.task_factory.guard();
	let task = guard
		.get(index.current_index())
		.context("The task you're trying to remove does not currently exist")?;
	let local = LocalStorage::new();
	match local.delete_task(task.clone().task.id).await {
		Ok(_) => {
			guard.remove(index.current_index());
			sender
				.output(ContentOutput::Notify(
					"Task removed successfully.".into(),
					1,
				))
				.unwrap_or_default();
		},
		Err(_) => {
			sender
				.output(ContentOutput::Notify("Error removing task.".into(), 2))
				.unwrap_or_default();
		},
	}

	Ok(())
}

pub async fn update_task(
	model: &mut ContentModel,
	sender: AsyncComponentSender<ContentModel>,
	task: Task,
) -> Result<()> {
	let local = LocalStorage::new();
	match local.update_task(task).await {
		Ok(_) => {
			if model.show_task_details {
				sender.input(ContentInput::HideFlap);
			}
			sender
				.output(ContentOutput::Notify("Task updated successfully".into(), 1))
				.unwrap_or_default()
		},
		Err(_) => {
			sender
				.output(ContentOutput::Notify(
					"An error ocurred while updating this task.".into(),
					2,
				))
				.unwrap_or_default();
		},
	}

	Ok(())
}

pub async fn select_task_list(
	model: &mut ContentModel,
	list: SidebarList,
) -> Result<()> {
	let local = LocalStorage::new();
	let mut guard = model.task_factory.guard();
	guard.clear();
	match list {
		SidebarList::All => {
			model.parent_list = None;
			if let Ok(response) = local.get_all_tasks().await {
				for task in response {
					guard.push_back(TaskInit::new(
						task.clone(),
						local.get_list(task.parent).await.ok(),
						model.compact,
					));
				}
			}
		},
		SidebarList::Today => {
			model.parent_list = None;
			if let Ok(response) = local.get_all_tasks().await {
				for task in response.iter().filter(|task| task.today) {
					guard.push_back(TaskInit::new(
						task.clone(),
						local.get_list(task.parent.clone()).await.ok(),
						model.compact,
					));
				}
			}
		},
		SidebarList::Starred => {
			model.parent_list = None;
			if let Ok(response) = local.get_all_tasks().await {
				for task in response.iter().filter(|task| task.favorite) {
					guard.push_back(TaskInit::new(
						task.clone(),
						local.get_list(task.parent.clone()).await.ok(),
						model.compact,
					));
				}
			}
		},
		SidebarList::Next7Days => {
			model.parent_list = None;
			if let Ok(response) = local.get_all_tasks().await {
				for task in response.iter().filter(|task: &&Task| {
					task.due_date.is_some()
						&& is_within_next_7_days(task.due_date.unwrap())
				}) {
					guard.push_back(TaskInit::new(
						task.clone(),
						local.get_list(task.parent.clone()).await.ok(),
						model.compact,
					));
				}
			}
		},
		SidebarList::Custom(list) => {
			model.parent_list = Some(list.clone());

			guard.clear();

			if let Ok(response) = local.get_tasks_from_list(list.id).await {
				for task in response {
					guard.push_back(TaskInit::new(
						task,
						model.parent_list.clone(),
						model.compact,
					));
				}
			}
		},
	}

	model
		.task_entry
		.sender()
		.send(TaskEntryInput::SetParentList(model.parent_list.clone()))
		.unwrap();

	Ok(())
}

fn is_within_next_7_days(date: NaiveDateTime) -> bool {
	let now = Utc::now().naive_utc();
	let next_7_days = now + chrono::Duration::days(7);
	date >= now && date <= next_7_days
}
