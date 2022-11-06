use adw::prelude::{ExpanderRowExt, PreferencesGroupExt, PreferencesRowExt};
use done_core::Channel;
use relm4::factory::{
	DynamicIndex, FactoryComponent, FactoryComponentSender, FactoryVecDeque,
	FactoryView,
};
use relm4::gtk;
use relm4::gtk::prelude::WidgetExt;
use relm4::ComponentController;
use relm4::{adw, Component, Controller};
use std::str::FromStr;

use done_core::plugins::{Plugin, PluginData};
use done_core::services::provider::provider_client::ProviderClient;
use done_core::services::provider::{Empty, List};

use crate::widgets::components::sidebar::SidebarInput;
use crate::widgets::factory::task_list::ListData;
use crate::widgets::popover::new_list::{NewListModel, NewListOutput};

#[allow(dead_code)]
#[derive(Debug)]
pub struct ProviderModel {
	pub provider: PluginData,
	pub list_factory: FactoryVecDeque<ListData>,
	pub new_list_controller: Controller<NewListModel>,
}

#[derive(Debug)]
pub enum ProviderInput {
	AddList(String, String),
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
}

#[relm4::factory(pub)]
impl FactoryComponent for ProviderModel {
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
				#[watch]
				set_enable_expansion: !self.list_factory.is_empty(),
				#[watch]
				set_expanded: !self.list_factory.is_empty(),
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

	fn init_model(
		params: Self::Init,
		_index: &DynamicIndex,
		sender: FactoryComponentSender<Self>,
	) -> Self {
		Self {
			provider: params.clone(),
			list_factory: FactoryVecDeque::new(
				adw::ExpanderRow::default(),
				sender.input_sender(),
			),
			new_list_controller: NewListModel::builder().launch(()).forward(
				sender.input_sender(),
				move |message| match message {
					NewListOutput::AddTaskListToSidebar(name) => {
						ProviderInput::AddList(params.id.clone(), name)
					},
				},
			),
		}
	}

	fn init_widgets(
		&mut self,
		index: &DynamicIndex,
		root: &Self::Root,
		_returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
		sender: FactoryComponentSender<Self>,
	) -> Self::Widgets {
		let widgets = view_output!();

		self.list_factory =
			FactoryVecDeque::new(widgets.expander.clone(), sender.input_sender());

		for list in &self.provider.lists {
			self.list_factory.guard().push_back(ListData { data: list.clone() });
		}

		widgets
	}

	fn update(
		&mut self,
		message: Self::Input,
		sender: FactoryComponentSender<Self>,
	) {
		match message {
			ProviderInput::DeleteTaskList(index) => {
				self.list_factory.guard().remove(index.current_index());
			},
			ProviderInput::AddList(provider_id, name) => {
				match Plugin::from_str(&provider_id) {
				    Ok(provider) => {
					    // let mut service = rt().block_on(provider.connect()).unwrap();
					    // let list = List::new(&name, "✍️", &provider_id);
					    // let response = rt()
						//     .block_on(service.create_list(list.clone()))
						//     .unwrap();
					    //
					    // if response.into_inner().successful {
						//     self.list_factory.guard().push_back(ListData { data: list });
					    // }
				    },
					Err(err) => eprintln!("{}", err),
				}
			},
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
			ProviderOutput::ProviderSelected(provider) => SidebarInput::ProviderSelected(provider)
		};
		Some(output)
	}
}
