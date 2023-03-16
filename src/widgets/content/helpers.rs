use anyhow::{anyhow, Context, Result};
use done_provider::Task;
use relm4::{prelude::DynamicIndex, AsyncComponentSender, ComponentController};

use crate::widgets::{
	details::model::TaskDetailsFactoryInit, task_entry::messages::TaskEntryInput,
	task_list::model::ListFactoryModel,
};

use super::{
	messages::{ContentInput, ContentOutput},
	model::{ContentModel, TaskInit},
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
		if let Some(plugin) = model.plugin.clone() {
			sender.input(ContentInput::TaskListSelected(ListFactoryModel::new(
				list, plugin,
			)))
		}
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
	let mut client = model
		.plugin
		.as_mut()
		.context("There is no plugin present, please select one first")?
		.connect()
		.await?;
	let response = client.create_task(task.clone()).await?.into_inner();
	if response.successful && response.task.is_some() {
		let task = response.task.unwrap();
		model.task_factory.guard().push_back(TaskInit::new(
			task,
			model.parent_list.as_ref().unwrap().clone(),
			model.compact,
		));
		model.show_task_details = false;
		sender.input(ContentInput::HideFlap);
		sender
			.output(ContentOutput::Notify(response.message, 1))
			.unwrap();
	} else {
		sender
			.output(ContentOutput::Notify(response.message, 2))
			.unwrap();
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
	let mut client = model
		.plugin
		.as_mut()
		.context("There is no plugin present, please select one first")?
		.connect()
		.await?;
	let response = client.delete_task(task.clone().task.id).await?.into_inner();

	if response.successful {
		guard.remove(index.current_index());
		sender
			.output(ContentOutput::Notify(response.message, 1))
			.unwrap_or_default();
	} else {
		sender
			.output(ContentOutput::Notify(response.message, 2))
			.unwrap_or_default();
	}
	Ok(())
}

pub async fn update_task(
	model: &mut ContentModel,
	sender: AsyncComponentSender<ContentModel>,
	task: Task,
) -> Result<()> {
	let mut client = model
		.plugin
		.as_mut()
		.context("There is no plugin present, please select one first")?
		.connect()
		.await?;
	let response = client.update_task(task).await?.into_inner();
	if response.successful {
		if model.show_task_details {
			sender.input(ContentInput::HideFlap);
		}
		sender
			.output(ContentOutput::Notify(response.message, 1))
			.unwrap_or_default()
	} else {
		sender
			.output(ContentOutput::Notify(response.message, 2))
			.unwrap_or_default();
	}
	Ok(())
}

pub async fn select_task_list(
	model: &mut ContentModel,
	list_model: ListFactoryModel,
) -> Result<()> {
	let (tx, mut rx) = relm4::tokio::sync::mpsc::channel(100);

	model.parent_list = Some(list_model.list.clone());
	model.plugin = Some(list_model.plugin.clone());
	model.selected_smart_list = None;
	model
		.task_entry
		.sender()
		.send(TaskEntryInput::SetParentList(model.parent_list.clone()))
		.unwrap();

	let mut client = list_model.plugin.connect().await?;
	let mut guard = model.task_factory.guard();

	guard.clear();

	relm4::spawn(async move {
		if let Ok(stream) = client.get_tasks_from_list(list_model.list.id).await {
			let mut stream = stream.into_inner();
			while let Some(response) = stream.message().await.unwrap() {
				tx.send(response).await.unwrap()
			}
		}
	});

	while let Some(response) = rx.recv().await {
		if response.successful && response.task.is_some() {
			guard.push_back(TaskInit::new(
				response.task.unwrap(),
				model.parent_list.as_ref().unwrap().clone(),
				model.compact,
			));
		}
	}
	Ok(())
}
