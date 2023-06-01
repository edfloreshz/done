use anyhow::{Context, Result};
use chrono::{NaiveDateTime, Utc};
use done_local_storage::models::{Status, Task};
use done_local_storage::services::Service;
use relm4::ComponentController;
use relm4::{prelude::DynamicIndex, AsyncComponentSender};

use crate::factories::task::model::TaskInit;
use crate::fl;
use crate::widgets::sidebar::model::SidebarList;
use crate::{
	factories::details::model::TaskDetailsFactoryInit,
	widgets::task_input::messages::TaskInputInput,
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
	service: Option<Service>,
) {
	model.show_task_details = false;
	if let Some(list) = &model.parent_list {
		sender.input(ContentInput::SelectList(list.clone(), service))
	}
}

pub async fn add_task(
	model: &mut ContentModel,
	sender: AsyncComponentSender<ContentModel>,
	task: &mut Task,
	service: Service,
) -> Result<()> {
	if let Some(SidebarList::Custom(parent)) = &model.parent_list {
		task.parent = parent.id.clone();
		let service = service.get_service();
		match service.create_task(task.clone()).await {
			Ok(_) => {
				model
					.task_factory
					.guard()
					.push_back(TaskInit::new(task.clone(), parent.clone()));
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
	}

	Ok(())
}

pub async fn remove_task(
	model: &mut ContentModel,
	sender: AsyncComponentSender<ContentModel>,
	index: DynamicIndex,
	service: Service,
) -> Result<()> {
	let mut guard = model.task_factory.guard();
	let task = guard
		.get(index.current_index())
		.context("The task you're trying to remove does not currently exist")?;
	let service = service.get_service();
	match service.delete_task(task.clone().task.id).await {
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
	service: Service,
) -> Result<()> {
	let service = service.get_service();
	match service.update_task(task).await {
		Ok(_) => {
			if model.show_task_details {
				sender.input(ContentInput::HideFlap);
			}
			sender.input(ContentInput::Refresh);
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
	service: Option<Service>,
) -> Result<()> {
	let mut guard = model.task_factory.guard();
	guard.clear();
	model.icon = list.icon().map(|s| s.to_owned());
	model.title = list.name();
	model.description = list.description();
	model.smart = list.smart();

	if let Some(service) = service {
		let service = service.get_service();
		match &list {
			SidebarList::All => {
				model.parent_list = Some(SidebarList::All);
				if let Ok(response) = service.read_tasks().await {
					for task in response {
						guard.push_back(TaskInit::new(
							task.clone(),
							service.read_list(task.parent).await.unwrap(),
						));
					}
				}
			},
			SidebarList::Today => {
				model.parent_list = Some(SidebarList::Today);
				if let Ok(response) = service.read_tasks().await {
					for task in response.iter().filter(|task| {
						task.today
							|| task.due_date.is_some()
								&& task.due_date.unwrap().date()
									== Utc::now().naive_utc().date()
					}) {
						guard.push_back(TaskInit::new(
							task.clone(),
							service.read_list(task.parent.clone()).await.unwrap(),
						));
					}
				}
			},
			SidebarList::Starred => {
				model.parent_list = Some(SidebarList::Starred);
				if let Ok(response) = service.read_tasks().await {
					for task in response.iter().filter(|task| task.favorite) {
						guard.push_back(TaskInit::new(
							task.clone(),
							service.read_list(task.parent.clone()).await.unwrap(),
						));
					}
				}
			},
			SidebarList::Next7Days => {
				model.parent_list = Some(SidebarList::Next7Days);
				if let Ok(response) = service.read_tasks().await {
					for task in response.iter().filter(|task: &&Task| {
						task.due_date.is_some()
							&& is_within_next_7_days(task.due_date.unwrap())
					}) {
						guard.push_back(TaskInit::new(
							task.clone(),
							service.read_list(task.parent.clone()).await.unwrap(),
						));
					}
				}
			},
			SidebarList::Done => {
				model.parent_list = Some(SidebarList::Done);
				if let Ok(response) = service.read_tasks().await {
					for task in response
						.iter()
						.filter(|task: &&Task| task.status == Status::Completed)
					{
						guard.push_back(TaskInit::new(
							task.clone(),
							service.read_list(task.parent.clone()).await.unwrap(),
						));
					}
				}
			},
			SidebarList::Custom(list) => {
				model.parent_list = Some(SidebarList::Custom(list.clone()));

				guard.clear();

				match service.read_tasks_from_list(list.id.clone()).await {
					Ok(response) => {
						for task in response
							.iter()
							.filter(|task| task.status != Status::Completed)
							.map(|task| task.to_owned())
						{
							guard.push_back(TaskInit::new(task, list.clone()));
						}
					},
					Err(err) => tracing::error!("{err}"),
				}
			},
		}
	}

	model.page_icon = if list.smart() {
		"/dev/edfloreshz/Done/icons/scalable/actions/empty.png"
	} else {
		"/dev/edfloreshz/Done/icons/scalable/actions/checked.png"
	};

	model.page_title = if list.smart() {
		fl!("list-empty").clone()
	} else {
		fl!("all-done").clone()
	};

	model.page_subtitle = if list.smart() {
		fl!("instructions").clone()
	} else {
		fl!("all-done-instructions").clone()
	};

	model
		.task_entry
		.sender()
		.send(TaskInputInput::SetParentList(model.parent_list.clone()))
		.unwrap();

	Ok(())
}

fn is_within_next_7_days(date: NaiveDateTime) -> bool {
	let now = Utc::now().naive_utc();
	let next_7_days = now + chrono::Duration::days(7);
	date >= now && date <= next_7_days
}
