use adw::prelude::{ExpanderRowExt, PreferencesGroupExt, PreferencesRowExt};
use done_core::plugins::{Plugin, PluginData};
use done_core::services::provider::List;
use relm4::factory::AsyncFactoryComponent;
use relm4::factory::AsyncFactoryVecDeque;
use relm4::factory::{AsyncFactorySender, DynamicIndex, FactoryView};
use relm4::gtk;
use relm4::gtk::prelude::WidgetExt;
use relm4::ComponentController;
use relm4::{adw, Component, Controller};

use crate::widgets::components::sidebar::SidebarInput;
use crate::widgets::factory::list::ListData;
use crate::widgets::popover::new_list::{NewListModel, NewListOutput};

#[allow(dead_code)]
#[derive(Debug)]
pub struct ProviderModel {
	pub is_connected: bool,
	pub provider: PluginData,
	pub list_factory: AsyncFactoryVecDeque<ListData>,
	pub new_list_controller: Controller<NewListModel>,
}

#[derive(Debug)]
pub enum ProviderInput {
	RequestAddList(usize, String, String),
	AddList(ListData),
	DeleteTaskList(DynamicIndex, String),
	Forward,
	ListSelected(List),
	SelectSmartProvider,
	Notify(String),
}

#[derive(Debug)]
pub enum ProviderOutput {
	ListSelected(List),
	ProviderSelected(Plugin),
	Forward,
	AddListToProvider(usize, String, String),
	Notify(String),
}

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for ProviderModel {
	type ParentInput = SidebarInput;
	type ParentWidget = gtk::Box;
	type CommandOutput = ();
	type Input = ProviderInput;
	type Output = ProviderOutput;
	type Init = Plugin;
	type Widgets = ProviderWidgets;

	view! {
		#[root]
		#[name(list_box)]
		adw::PreferencesGroup {
			#[name(expander)]
			add = &adw::ExpanderRow {
				#[watch]
				set_title: self.provider.name.as_str(),
				#[watch]
				set_subtitle: self.provider.description.as_str(),
				#[watch]
				set_icon_name: Some(self.provider.icon.as_str()),
				#[watch]
				set_enable_expansion: !self.provider.lists.is_empty() && self.is_connected,
				set_expanded: !self.provider.lists.is_empty(),
				add_action = if self.is_connected {
					gtk::MenuButton {
						set_icon_name: "value-increase-symbolic",
						set_css_classes: &["flat", "image-button"],
						set_valign: gtk::Align::Center,
						set_direction: gtk::ArrowType::Right,
						set_popover: Some(self.new_list_controller.widget())
					}
								} else {
										gtk::Spinner {
										start: (),
										set_hexpand: false,
									}
								},
			},
			add_controller = &gtk::GestureClick {
				connect_pressed[sender, index] => move |_, _, _, _| {
					if index.clone().current_index() <= 3 {
						sender.input(ProviderInput::SelectSmartProvider);
						sender.input(ProviderInput::Forward)
					}
				}
			}
		}
	}

	async fn init_model(
		init: Self::Init,
		index: &DynamicIndex,
		sender: AsyncFactorySender<Self>,
	) -> Self {
		let provider = init.data().await.unwrap();
		let index = index.current_index();
		Self {
			is_connected: true,
			provider: provider.clone(),
			list_factory: AsyncFactoryVecDeque::new(
				adw::ExpanderRow::default(),
				sender.input_sender(),
			),
			new_list_controller: NewListModel::builder().launch(()).forward(
				sender.input_sender(),
				move |message| match message {
					NewListOutput::AddTaskListToSidebar(name) => {
						ProviderInput::RequestAddList(index, provider.id.clone(), name)
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
		sender: AsyncFactorySender<Self>,
	) -> Self::Widgets {
		let widgets = view_output!();

		self.list_factory = AsyncFactoryVecDeque::new(
			widgets.expander.clone(),
			sender.input_sender(),
		);

		for list in &self.provider.lists {
			self
				.list_factory
				.guard()
				.push_back(ListData { data: list.clone() });
		}

		widgets
	}

	async fn update(
		&mut self,
		message: Self::Input,
		sender: AsyncFactorySender<Self>,
	) {
		match message {
			ProviderInput::DeleteTaskList(index, list_id) => {
				self.list_factory.guard().remove(index.current_index());
				let index = self
					.provider
					.lists
					.iter()
					.position(|list| list.id == list_id)
					.unwrap();
				self.provider.lists.remove(index);
				self.provider = self.provider.plugin.data().await.unwrap();
				info!("Deleted task list with id: {}", list_id);
			},
			ProviderInput::RequestAddList(index, provider_id, name) => sender
				.output(ProviderOutput::AddListToProvider(index, provider_id, name)),
			ProviderInput::AddList(list) => {
				self.list_factory.guard().push_back(list);
				self.provider = self.provider.plugin.data().await.unwrap();
				info!("List added to {}", self.provider.name)
			},
			ProviderInput::Forward => sender.output(ProviderOutput::Forward),
			ProviderInput::ListSelected(list) => {
				sender.output(ProviderOutput::ListSelected(list.clone()));
				info!("List selected: {}", list.name)
			},
			ProviderInput::SelectSmartProvider => {
				sender.output(ProviderOutput::ProviderSelected(self.provider.plugin));
				info!("Provider selected: {}", self.provider.name)
			},
			ProviderInput::Notify(msg) => sender.output(ProviderOutput::Notify(msg)),
		}
	}

	fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
		let output = match output {
			ProviderOutput::ListSelected(list) => SidebarInput::ListSelected(list),
			ProviderOutput::Forward => SidebarInput::Forward,
			ProviderOutput::ProviderSelected(provider) => {
				SidebarInput::ProviderSelected(provider)
			},
			ProviderOutput::AddListToProvider(index, provider_id, name) => {
				SidebarInput::AddListToProvider(index, provider_id, name)
			},
			ProviderOutput::Notify(msg) => SidebarInput::Notify(msg),
		};
		Some(output)
	}
}
