use crate::application::plugin::Plugin;
use crate::widgets::factory::list::ListInit;
use adw::prelude::{ExpanderRowExt, PreferencesRowExt};
use libset::format::FileFormat;
use libset::project::Project;
use proto_rust::provider::List;
use proto_rust::provider_client::ProviderClient;
use proto_rust::Channel;
use relm4::factory::AsyncFactoryComponent;
use relm4::factory::AsyncFactoryVecDeque;
use relm4::factory::{AsyncFactorySender, DynamicIndex, FactoryView};
use relm4::gtk;
use relm4::gtk::prelude::WidgetExt;
use relm4::ComponentController;
use relm4::{adw, Component, Controller};

use crate::widgets::components::preferences::Preferences;
use crate::widgets::components::sidebar::SidebarInput;
use crate::widgets::factory::list::ListData;
use crate::widgets::popover::new_list::{NewListModel, NewListOutput};

#[allow(dead_code)]
#[derive(Debug)]
pub struct ProviderModel {
	pub plugin: Plugin,
	pub service: ProviderClient<Channel>,
	pub enabled: bool,
	pub lists: Vec<String>,
	pub list_factory: AsyncFactoryVecDeque<ListData>,
	pub new_list_controller: Controller<NewListModel>,
}

#[derive(derive_new::new)]
pub struct PluginInit {
	plugin: Plugin,
	service: ProviderClient<Channel>,
}

#[derive(Debug)]
pub enum ProviderInput {
	RequestAddList(usize, String),
	AddList(List),
	DeleteTaskList(DynamicIndex, String),
	Forward,
	ListSelected(ListData),
	Notify(String),
	Enable,
	Disable,
}

#[derive(Debug)]
pub enum ProviderOutput {
	AddListToProvider(usize, String, String),
	ListSelected(ListData),
	Notify(String),
	Forward,
}

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for ProviderModel {
	type ParentInput = SidebarInput;
	type ParentWidget = adw::PreferencesGroup;
	type CommandOutput = ();
	type Input = ProviderInput;
	type Output = ProviderOutput;
	type Init = PluginInit;
	type Widgets = ProviderWidgets;

	view! {
		#[root]
		adw::ExpanderRow {
			#[watch]
			set_title: self.plugin.name.as_str(),
			#[watch]
			set_subtitle: self.plugin.description.as_str(),
			#[watch]
			set_icon_name: Some(self.plugin.icon.as_str()),
			#[watch]
			set_enable_expansion: !self.lists.is_empty() && self.plugin.is_running() && self.enabled,
			set_expanded: !self.lists.is_empty(),
			add_action = if self.plugin.is_running() {
				gtk::MenuButton {
					set_icon_name: "value-increase-symbolic",
					set_css_classes: &["flat", "image-button"],
					set_valign: gtk::Align::Center,
					set_direction: gtk::ArrowType::Right,
					set_popover: Some(self.new_list_controller.widget())
				}
			} else {
				gtk::Box {

				}
			},
		}
	}

	async fn init_model(
		init: Self::Init,
		index: &DynamicIndex,
		sender: AsyncFactorySender<Self>,
	) -> Self {
		let plugin_preferences = Project::open("dev", "edfloreshz", "done")
			.unwrap()
			.get_file_as::<Preferences>("preferences", FileFormat::JSON)
			.unwrap()
			.plugins;
		let index = index.current_index();
		Self {
			plugin: init.plugin.clone(),
			service: init.service,
			enabled: plugin_preferences
				.iter()
				.find(|p| p.plugin.name == init.plugin.name)
				.unwrap()
				.enabled,
			lists: init.plugin.lists().await.unwrap(),
			list_factory: AsyncFactoryVecDeque::new(
				adw::ExpanderRow::default(),
				sender.input_sender(),
			),
			new_list_controller: NewListModel::builder().launch(()).forward(
				sender.input_sender(),
				move |message| match message {
					NewListOutput::AddTaskListToSidebar(name) => {
						ProviderInput::RequestAddList(index, name)
					},
				},
			),
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

		self.list_factory =
			AsyncFactoryVecDeque::new(root.clone(), sender.input_sender());

		for list in &self.lists {
			self
				.list_factory
				.guard()
				.push_back(ListInit::new(list.clone(), self.service.clone()));
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
					.lists
					.iter()
					.position(|list_id| list_id == list_id)
					.unwrap();
				self.lists.remove(index);
				info!("Deleted task list with id: {}", list_id);
			},
			ProviderInput::RequestAddList(index, name) => sender.output(
				ProviderOutput::AddListToProvider(index, self.plugin.id.clone(), name),
			),
			ProviderInput::AddList(list) => {
				self
					.list_factory
					.guard()
					.push_back(ListInit::new(list.id.clone(), self.service.clone()));
				self.lists.push(list.id);
				info!("List added to {}", self.plugin.name)
			},
			ProviderInput::Forward => sender.output(ProviderOutput::Forward),
			ProviderInput::ListSelected(list) => {
				sender.output(ProviderOutput::ListSelected(list.clone()));
				info!("List selected: {}", list.list.name)
			},
			ProviderInput::Notify(msg) => sender.output(ProviderOutput::Notify(msg)),
			ProviderInput::Enable => {
				self.enabled = true;

				self.list_factory.guard().clear();
				for list in &self.lists {
					self
						.list_factory
						.guard()
						.push_back(ListInit::new(list.clone(), self.service.clone()));
				}
			},
			ProviderInput::Disable => self.enabled = false,
		}
	}

	fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
		let output = match output {
			ProviderOutput::ListSelected(list) => SidebarInput::ListSelected(list),
			ProviderOutput::Forward => SidebarInput::Forward,
			ProviderOutput::AddListToProvider(index, provider_id, name) => {
				SidebarInput::AddListToProvider(index, provider_id, name)
			},
			ProviderOutput::Notify(msg) => SidebarInput::Notify(msg),
		};
		Some(output)
	}
}
