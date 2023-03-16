use crate::{
	application::plugin::Plugin,
	widgets::{
		plugin::{messages::PluginFactoryInput, model::PluginFactoryInit},
		preferences::model::Preferences,
	},
};
use anyhow::Result;
use done_provider::List;
use libset::{format::FileFormat, project::Project};
use relm4::AsyncComponentSender;

use super::{messages::SidebarComponentOutput, model::SidebarComponentModel};

pub async fn add_list_to_provider(
	model: &mut SidebarComponentModel,
	sender: AsyncComponentSender<SidebarComponentModel>,
	index: usize,
	plugin: Plugin,
	name: String,
) -> Result<()> {
	let mut client = plugin.connect().await?;
	let list = List::new(&name);
	let create_list = list.clone();
	let response = client.create_list(create_list).await?.into_inner();
	if response.successful {
		model
			.plugin_factory
			.send(index, PluginFactoryInput::AddList(list));
		sender
			.output(SidebarComponentOutput::Notify(response.message, 1))
			.unwrap_or_default();
	} else {
		tracing::error!(response.message);
		sender
			.output(SidebarComponentOutput::Notify(response.message, 2))
			.unwrap_or_default();
	}
	Ok(())
}

pub async fn add_plugin_to_sidebar(
	model: &mut SidebarComponentModel,
	plugin: Plugin,
) -> Result<()> {
	plugin.start().await?;
	if plugin.connect().await.is_ok() {
		model
			.plugin_factory
			.guard()
			.push_back(PluginFactoryInit::new(plugin.clone(), true));
		model.is_sidebar_empty = false;
		tracing::info!("Added {:?} service to the sidebar", plugin.name);
	}
	Ok(())
}

pub fn enable_service(model: &mut SidebarComponentModel, plugin: Plugin) {
	let index = model
		.plugin_factory
		.guard()
		.iter()
		.position(|p| p.map_or(false, |p| p.plugin == plugin));
	if let Some(index) = index {
		model.plugin_factory.send(index, PluginFactoryInput::Enable);
	}
}

pub fn disable_service(
	model: &mut SidebarComponentModel,
	sender: AsyncComponentSender<SidebarComponentModel>,
	plugin: Plugin,
) -> Result<()> {
	let index = model
		.plugin_factory
		.guard()
		.iter()
		.position(|p| p.unwrap().plugin == plugin);
	if let Some(index) = index {
		model
			.plugin_factory
			.send(index, PluginFactoryInput::Disable);
		sender
			.output(SidebarComponentOutput::DisablePlugin)
			.unwrap_or_default();
		let project = Project::open("dev", "edfloreshz", "done")?;
		let preferences =
			project.get_file_as::<Preferences>("preferences", FileFormat::JSON)?;
		model.is_sidebar_empty = !preferences
			.plugins
			.iter()
			.any(|preferences| preferences.installed);
	}
	Ok(())
}

pub fn remove_service(
	model: &mut SidebarComponentModel,
	plugin: Plugin,
) -> Result<()> {
	let index = model
		.plugin_factory
		.guard()
		.iter()
		.position(|p| p.unwrap().plugin == plugin);
	if let Some(index) = index {
		match model.plugin_factory.guard().remove(index) {
			Some(provider) => {
				tracing::info!("Removed {} service", provider.plugin.name)
			},
			None => tracing::error!("Failed to remove service from sidebar."),
		}
	}
	if model.plugin_factory.guard().is_empty() {
		model.is_sidebar_empty = true;
	}
	Ok(())
}
