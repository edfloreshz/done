use anyhow::Result;
use done_provider::List;
use relm4::AsyncComponentSender;

use crate::{
	application::plugin::Plugin,
	factories::task_list::model::TaskListFactoryInit,
	widgets::lists::messages::TaskListsOutput,
};

use super::model::TaskListsModel;

pub async fn add_list_to_provider(
	model: &mut TaskListsModel,
	sender: AsyncComponentSender<TaskListsModel>,
	plugin: Plugin,
	name: String,
) -> Result<()> {
	let mut client = plugin.connect().await?;
	let list = List::new(&name);
	let create_list = list.clone();
	let response = client.create_list(create_list).await?.into_inner();
	if response.successful {
		model
			.list_factory
			.guard()
			.push_back(TaskListFactoryInit::new(plugin, list));
	} else {
		tracing::error!(response.message);
		sender
			.output(TaskListsOutput::Notify(response.message))
			.unwrap_or_default();
	}
	Ok(())
}
