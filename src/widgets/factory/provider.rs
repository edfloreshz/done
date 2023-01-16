use crate::application::plugin::{Plugin, PluginData};
use crate::widgets::factory::list::ListInit;
use adw::prelude::{ExpanderRowExt, PreferencesGroupExt, PreferencesRowExt};
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
use relm4::loading_widgets::LoadingWidgets;
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
	pub data: PluginData,
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
	type ParentWidget = gtk::Box;
	type CommandOutput = ();
	type Input = ProviderInput;
	type Output = ProviderOutput;
	type Init = PluginInit;
	type Widgets = ProviderWidgets;

	view! {
		#[root]
		#[name(list_box)]
		adw::PreferencesGroup {
			#[name(expander)]
			add = &adw::ExpanderRow {
				#[watch]
				set_title: self.data.name.as_str(),
				#[watch]
				set_subtitle: self.data.description.as_str(),
				#[watch]
				set_icon_name: Some(self.data.icon.as_str()),
				#[watch]
				set_enable_expansion: !self.data.lists.is_empty() && self.plugin.is_running() && self.enabled,
				set_expanded: !self.data.lists.is_empty(),
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
			},
		}
	}

	fn init_loading_widgets(
		root: &mut Self::Root,
	) -> Option<relm4::loading_widgets::LoadingWidgets> {
		relm4::view! {
			#[local_ref]
			root {
				#[name(expander)]
				add = &adw::ExpanderRow {

				}
			}
		}
		Some(LoadingWidgets::new(root, expander))
	}

	async fn init_model(
		init: Self::Init,
		index: &DynamicIndex,
		sender: AsyncFactorySender<Self>,
	) -> Self {
		let plugin_preferences = Project::open("dev", "edfloreshz", "done")
			.unwrap()
			.get_file_as::<Preferences>("preferences", FileFormat::TOML)
			.unwrap()
			.plugins;
		let data = if init.plugin.is_running() {
			init.plugin.data().await.unwrap()
		} else {
			init.plugin.placeholder()
		};
		let enabled = match init.plugin {
			Plugin::Local => plugin_preferences.local_enabled,
			Plugin::Google => plugin_preferences.google_enabled,
			Plugin::Microsoft => plugin_preferences.microsoft_enabled,
			Plugin::Nextcloud => plugin_preferences.nextcloud_enabled,
		};
		let index = index.current_index();
		Self {
			plugin: init.plugin,
			service: init.service,
			enabled,
			data,
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

		self.list_factory = AsyncFactoryVecDeque::new(
			widgets.expander.clone(),
			sender.input_sender(),
		);

		for list in &self.data.lists {
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
					.data
					.lists
					.iter()
					.position(|list| list.id == list_id)
					.unwrap();
				self.data.lists.remove(index);
				self.data = self.plugin.data().await.unwrap();
				info!("Deleted task list with id: {}", list_id);
			},
			ProviderInput::RequestAddList(index, name) => {
				sender.output(ProviderOutput::AddListToProvider(
					index,
					self.plugin.data().await.unwrap().id,
					name,
				))
			},
			ProviderInput::AddList(list) => {
				self
					.list_factory
					.guard()
					.push_back(ListInit::new(list, self.service.clone()));
				self.data = self.plugin.data().await.unwrap();
				info!("List added to {}", self.data.name)
			},
			ProviderInput::Forward => sender.output(ProviderOutput::Forward),
			ProviderInput::ListSelected(list) => {
				sender.output(ProviderOutput::ListSelected(list.clone()));
				info!("List selected: {}", list.list.name)
			},
			ProviderInput::Notify(msg) => sender.output(ProviderOutput::Notify(msg)),
			ProviderInput::Enable => {
				self.enabled = true;
				self.data = self.plugin.data().await.unwrap();
				loop {
					let list = self.list_factory.guard().pop_front();
					if list.is_none() {
						break;
					}
				}
				for list in &self.data.lists {
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
