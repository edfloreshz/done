use adw::prelude::{ExpanderRowExt, PreferencesGroupExt, PreferencesRowExt};
use relm4::factory::{AsyncFactoryComponentSender, DynamicIndex, FactoryView};
use relm4::gtk;
use relm4::gtk::prelude::WidgetExt;
use relm4::ComponentController;
use relm4::{adw, Component, Controller};
use relm4::factory::r#async::collections::AsyncFactoryVecDeque;
use relm4::factory::r#async::traits::AsyncFactoryComponent;

use done_core::plugins::{Plugin, PluginData};
use done_core::services::provider::List;

use crate::widgets::components::sidebar::SidebarInput;
use crate::widgets::factory::list::ListData;
use crate::widgets::popover::new_list::{NewListModel, NewListOutput};

#[allow(dead_code)]
#[derive(Debug)]
pub struct ProviderModel {
	pub provider: PluginData,
	pub list_factory: AsyncFactoryVecDeque<ListData>,
	pub new_list_controller: Controller<NewListModel>,
}

#[derive(Debug)]
pub enum ProviderInput {
	RequestAddList(usize, String, String),
	AddList(ListData),
	DeleteTaskList(DynamicIndex),
	Forward(bool),
	ListSelected(List),
	SelectSmartProvider,
}

#[derive(Debug)]
pub enum ProviderOutput {
	ListSelected(List),
	ProviderSelected(Plugin),
	Forward,
	AddListToProvider(usize, String, String)
}

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for ProviderModel {
	type ParentInput = SidebarInput;
	type ParentWidget = gtk::Box;
	type CommandOutput = ();
	type Input = ProviderInput;
	type Output = ProviderOutput;
	type Init = PluginData;
	type Widgets = ProviderWidgets;

	view! {
		#[root]
		#[name(list_box)]
		adw::PreferencesGroup {
			#[name(expander)]
			add = &adw::ExpanderRow {
				set_title: self.provider.name.as_str(),
				set_subtitle: self.provider.description.as_str(),
				set_icon_name: Some(self.provider.icon.as_str()),
				set_enable_expansion: !self.provider.lists.is_empty(),
				set_expanded: !self.provider.lists.is_empty(),
				add_action = &gtk::MenuButton {
					set_icon_name: "value-increase-symbolic",
					set_css_classes: &["flat", "image-button"],
					set_valign: gtk::Align::Center,
					set_direction: gtk::ArrowType::Right,
					set_popover: Some(self.new_list_controller.widget())
				},
			},
			add_controller = &gtk::GestureClick {
				connect_pressed[sender, index] => move |_, _, _, _| {
					if index.clone().current_index() <= 3 {
						sender.input(ProviderInput::SelectSmartProvider);
					}
					sender.input(ProviderInput::Forward(index.clone().current_index() <= 3))
				}
			}
		}
	}

	async fn init_model(init: Self::Init, index: &DynamicIndex, sender: AsyncFactoryComponentSender<Self>) -> Self {
		let index = index.current_index();
		Self {
			provider: init.clone(),
			list_factory: AsyncFactoryVecDeque::new(
				adw::ExpanderRow::default(),
				sender.input_sender(),
			),
			new_list_controller: NewListModel::builder().launch(()).forward(
				sender.input_sender(),
				move |message| match message {
					NewListOutput::AddTaskListToSidebar(name) => {
						ProviderInput::RequestAddList(index, init.id.clone(), name)
					},
				},
			),
		}
	}

	fn init_widgets(&mut self, index: &DynamicIndex, root: &Self::Root, _returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget, sender: AsyncFactoryComponentSender<Self>) -> Self::Widgets {
		let widgets = view_output!();

		self.list_factory =
			AsyncFactoryVecDeque::new(widgets.expander.clone(), sender.input_sender());

		for list in &self.provider.lists {
			self.list_factory.guard().push_back(ListData { data: list.clone() });
		}

		widgets
	}

	async fn update(
		&mut self,
		message: Self::Input,
		sender: AsyncFactoryComponentSender<Self>,
	) {
		match message {
			ProviderInput::DeleteTaskList(index) => {
				self.list_factory.guard().remove(index.current_index());
			},
			ProviderInput::RequestAddList(index, provider_id, name) => {
				sender.output(ProviderOutput::AddListToProvider(index, provider_id, name))
			},
			ProviderInput::AddList(list) => {
				self.list_factory.guard().push_back(list);
			}
			ProviderInput::Forward(forward) => {
				if forward {
					sender.output(ProviderOutput::Forward)
				}
			},
			ProviderInput::ListSelected(list) => {
				sender.output(ProviderOutput::ListSelected(list))
			},
			ProviderInput::SelectSmartProvider => {
				sender.output(ProviderOutput::ProviderSelected(self.provider.plugin));
			},
		}
	}

	fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
		let output = match output {
			ProviderOutput::ListSelected(list) => SidebarInput::ListSelected(list),
			ProviderOutput::Forward => SidebarInput::Forward,
			ProviderOutput::ProviderSelected(provider) => SidebarInput::ProviderSelected(provider),
			ProviderOutput::AddListToProvider(index, provider_id, name) => {
				SidebarInput::AddListToProvider(index, provider_id, name)
			}
		};
		Some(output)
	}
}
