use crate::{
	application::plugin::Plugin,
	factories::plugin::{messages::PluginFactoryInput, model::PluginFactoryInit},
};
use anyhow::Result;
use relm4::AsyncComponentSender;

use super::{messages::SidebarComponentOutput, model::SidebarComponentModel};

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
	Ok(())
}
