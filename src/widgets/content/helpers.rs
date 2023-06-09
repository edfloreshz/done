use anyhow::Result;
use chrono::{DateTime, Utc};
use done_local_storage::models::{status::Status, task::Task};
use done_local_storage::service::Service;
use relm4::ComponentController;

use crate::factories::task::model::TaskInit;
use crate::widgets::sidebar::model::SidebarList;
use crate::widgets::task_input::messages::TaskInputInput;

use super::widget::{ContentModel, ContentState};

pub async fn select_task_list(
	model: &mut ContentModel,
	list: SidebarList,
	service: Service,
) -> Result<()> {
	let mut guard = model.task_factory.guard();
	guard.clear();
	model.service = service;

	let mut service = service.get_service();
	match &list {
		SidebarList::All => {
			model.parent_list = SidebarList::All;
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
			model.parent_list = SidebarList::Today;
			if let Ok(response) = service.read_tasks().await {
				for task in response.iter().filter(|task| {
					task.today
						|| task.due_date.is_some()
							&& task.due_date.unwrap().date_naive() == Utc::now().date_naive()
				}) {
					guard.push_back(TaskInit::new(
						task.clone(),
						service.read_list(task.parent.clone()).await.unwrap(),
					));
				}
			}
		},
		SidebarList::Starred => {
			model.parent_list = SidebarList::Starred;
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
			model.parent_list = SidebarList::Next7Days;
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
			model.parent_list = SidebarList::Done;
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
			model.parent_list = SidebarList::Custom(list.clone());

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

	model.state = ContentState::TasksLoaded;

	if guard.is_empty() {
		model.state = ContentState::AllDone;
	}

	if list.smart() {
		model.state = ContentState::Empty;
	}

	model
		.task_entry
		.sender()
		.send(TaskInputInput::SetParentList(model.parent_list.clone()))
		.unwrap();

	Ok(())
}

fn is_within_next_7_days(date: DateTime<Utc>) -> bool {
	let now = Utc::now();
	let next_7_days = now + chrono::Duration::days(7);
	date >= now && date <= next_7_days
}
