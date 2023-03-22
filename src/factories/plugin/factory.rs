use crate::widgets::sidebar::messages::SidebarComponentInput;
use relm4::factory::AsyncFactoryComponent;
use relm4::factory::{AsyncFactorySender, DynamicIndex, FactoryView};
use relm4::gtk;
use relm4::gtk::prelude::ListBoxRowExt;
use relm4::gtk::traits::WidgetExt;

use super::messages::{PluginFactoryInput, PluginFactoryOutput};
use super::model::{PluginFactoryInit, PluginFactoryModel};

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for PluginFactoryModel {
	type ParentInput = SidebarComponentInput;
	type ParentWidget = gtk::ListBox;
	type CommandOutput = ();
	type Input = PluginFactoryInput;
	type Output = PluginFactoryOutput;
	type Init = PluginFactoryInit;

	view! {
		#[root]
		gtk::ListBoxRow {
			set_has_tooltip: true,
			set_tooltip_text: Some(&self.plugin.name),
			gtk::CenterBox {
				set_css_classes: &["plugin"],
				#[wrap(Some)]
				set_center_widget = &gtk::Image {
					set_icon_name: Some(self.plugin.icon.as_str())
				}
			},
			connect_activate => PluginFactoryInput::PluginSelected
		}
	}

	async fn init_model(
		init: Self::Init,
		_index: &DynamicIndex,
		_sender: AsyncFactorySender<Self>,
	) -> Self {
		Self {
			plugin: init.plugin,
			enabled: init.enabled,
		}
	}

	fn init_widgets(
		&mut self,
		_index: &DynamicIndex,
		root: &Self::Root,
		_returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
		sender: AsyncFactorySender<Self>,
	) -> Self::Widgets {
		let widgets = view_output!();
		widgets
	}

	async fn update(
		&mut self,
		message: Self::Input,
		sender: AsyncFactorySender<Self>,
	) {
		match message {
			PluginFactoryInput::PluginSelected => {
				sender.output(PluginFactoryOutput::PluginSelected(self.plugin.clone()));
				tracing::info!("Plugin selected: {}", self.plugin.name);
			},
			PluginFactoryInput::Enable => self.enabled = true,
			PluginFactoryInput::Disable => self.enabled = false,
		}
	}

	fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
		let output = match output {
			PluginFactoryOutput::PluginSelected(plugin) => {
				SidebarComponentInput::PluginSelected(plugin)
			},
		};
		Some(output)
	}
}
