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

use done_core::plugins::Plugin;
use done_core::provider::provider_client::ProviderClient;
use done_core::provider::{Empty, List, ProviderRequest};

use crate::rt;
use crate::widgets::components::sidebar::SidebarInput;
use crate::widgets::factory::task_list::ListData;
use crate::widgets::popover::new_list::{NewListModel, NewListOutput};

#[allow(dead_code)]
#[derive(Debug)]
pub struct ProviderModel {
	pub provider: Plugin,
	pub connector: ProviderClient<Channel>,
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
	type Init = (Plugin, ProviderClient<Channel>);
	type Widgets = ProviderWidgets;

	view! {
		#[root]
		#[name(list_box)]
		adw::PreferencesGroup {
			#[name(expander)]
			add = &adw::ExpanderRow {
				set_title: rt().block_on(self.connector.get_name(Empty {})).unwrap().into_inner().as_str(),
				set_subtitle: rt().block_on(self.connector.get_description(Empty {})).unwrap().into_inner().as_str(),
				set_icon_name: Some(rt().block_on(self.connector.get_icon_name(Empty {})).unwrap().into_inner().as_str()),
				set_enable_expansion: true,
				set_expanded: false,
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
		let mut service = params.1;
		let id = rt()
			.block_on(service.get_id(Empty {}))
			.unwrap()
			.into_inner();

		Self {
			provider: params.0,
			connector: service,
			list_factory: FactoryVecDeque::new(
				adw::ExpanderRow::default(),
				&sender.input_sender(),
			),
			new_list_controller: NewListModel::builder().launch(()).forward(
				&sender.input_sender(),
				move |message| match message {
					NewListOutput::AddTaskListToSidebar(name) => {
						ProviderInput::AddList(id.clone(), name)
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

		let response = rt()
			.block_on(self.connector.read_all_lists(Empty {}))
			.unwrap()
			.into_inner();
		let data: Vec<List> =
			serde_json::from_str(response.data.unwrap().as_str()).unwrap();
		for list in data {
			self.list_factory.guard().push_back(ListData { data: list });
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
					    let mut service = rt().block_on(provider.connect()).unwrap();
					    let list = List::new(&name, "✍️", &provider_id);
					    let response = rt()
						    .block_on(service.create_list(ProviderRequest {
							    list: Some(list.clone()),
							    task: None,
						    }))
						    .unwrap();

					    if response.into_inner().successful {
						    self.list_factory.guard().push_back(ListData { data: list });
					    }
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
				sender.output(ProviderOutput::ProviderSelected(self.provider));
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
