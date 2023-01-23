use crate::application::plugin::Plugin;
use crate::widgets::factory::list::ListFactoryInit;
use adw::prelude::{ExpanderRowExt, PreferencesRowExt};
use proto_rust::provider::List;
use proto_rust::Empty;
use proto_rust::ListResponse;
use relm4::factory::AsyncFactoryComponent;
use relm4::factory::AsyncFactoryVecDeque;
use relm4::factory::{AsyncFactorySender, DynamicIndex, FactoryView};
use relm4::gtk;
use relm4::gtk::prelude::WidgetExt;
use relm4::tokio::sync::mpsc::Receiver;
use relm4::ComponentController;
use relm4::{adw, Component, Controller};

use crate::widgets::components::list_entry::{ListEntryModel, ListEntryOutput};
use crate::widgets::components::sidebar::SidebarComponentInput;
use crate::widgets::factory::list::ListFactoryModel;

#[allow(dead_code)]
#[derive(Debug)]
pub struct PluginFactoryModel {
	pub plugin: Plugin,
	pub enabled: bool,
	pub last_list_selected: Option<ListFactoryModel>,
	pub list_factory: AsyncFactoryVecDeque<ListFactoryModel>,
	pub new_list_controller: Controller<ListEntryModel>,
	pub rx: Receiver<ListResponse>,
}

#[derive(Debug)]
pub enum PluginFactoryInput {
	FillTaskFactory,
	RequestAddList(usize, String),
	AddList(List),
	DeleteTaskList(DynamicIndex, String),
	Forward,
	ListSelected(ListFactoryModel),
	Notify(String),
	Enable,
	Disable,
}

#[derive(Debug)]
pub enum PluginFactoryOutput {
	AddListToProvider(usize, Plugin, String),
	ListSelected(ListFactoryModel),
	Notify(String),
	Forward,
}

#[derive(derive_new::new)]
pub struct PluginFactoryInit {
	plugin: Plugin,
	enabled: bool,
}

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for PluginFactoryModel {
	type ParentInput = SidebarComponentInput;
	type ParentWidget = adw::PreferencesGroup;
	type CommandOutput = ();
	type Input = PluginFactoryInput;
	type Output = PluginFactoryOutput;
	type Init = PluginFactoryInit;

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
			set_enable_expansion: !self.list_factory.is_empty() && self.plugin.is_running() && self.enabled,
			set_expanded: !self.list_factory.is_empty(),
			add_action = &gtk::MenuButton {
				#[watch]
				set_visible: self.enabled,
				set_icon_name: "value-increase-symbolic",
				set_css_classes: &["flat", "image-button"],
				set_valign: gtk::Align::Center,
				set_direction: gtk::ArrowType::Right,
				set_popover: Some(self.new_list_controller.widget())
			},
		}
	}

	async fn init_model(
		init: Self::Init,
		index: &DynamicIndex,
		sender: AsyncFactorySender<Self>,
	) -> Self {
		match init_model(init, index, sender).await {
			Ok(model) => return model,
			Err(err) => {
				tracing::error!("{err}");
				panic!("{err}") //TODO: Handle this better
			},
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
		sender.input(PluginFactoryInput::FillTaskFactory);
		widgets
	}

	async fn update(
		&mut self,
		message: Self::Input,
		sender: AsyncFactorySender<Self>,
	) {
		match message {
			PluginFactoryInput::FillTaskFactory => {
				while let Some(response) = self.rx.recv().await {
					if response.successful {
						self.list_factory.guard().push_back(ListFactoryInit::new(
							self.plugin.clone(),
							response.list.unwrap(),
						));
					}
				}
			},
			PluginFactoryInput::DeleteTaskList(index, list_id) => {
				self.list_factory.guard().remove(index.current_index());
				tracing::info!("Deleted task list with id: {}", list_id);
			},
			PluginFactoryInput::RequestAddList(index, name) => {
				sender.output(PluginFactoryOutput::AddListToProvider(
					index,
					self.plugin.clone(),
					name,
				))
			},
			PluginFactoryInput::AddList(list) => {
				self
					.list_factory
					.guard()
					.push_back(ListFactoryInit::new(self.plugin.clone(), list));
				tracing::info!("List added to {}", self.plugin.name);
			},
			PluginFactoryInput::Forward => {
				sender.output(PluginFactoryOutput::Forward)
			},
			PluginFactoryInput::ListSelected(model) => {
				sender.output(PluginFactoryOutput::ListSelected(model.clone()));
				tracing::info!("List selected: {}", model.list.name);
			},
			PluginFactoryInput::Notify(msg) => {
				sender.output(PluginFactoryOutput::Notify(msg))
			},
			PluginFactoryInput::Enable => self.enabled = true,
			PluginFactoryInput::Disable => self.enabled = false,
		}
	}

	fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
		let output = match output {
			PluginFactoryOutput::ListSelected(list) => {
				SidebarComponentInput::ListSelected(list)
			},
			PluginFactoryOutput::Forward => SidebarComponentInput::Forward,
			PluginFactoryOutput::AddListToProvider(index, plugin, name) => {
				SidebarComponentInput::AddListToProvider(index, plugin, name)
			},
			PluginFactoryOutput::Notify(msg) => SidebarComponentInput::Notify(msg),
		};
		Some(output)
	}
}

async fn init_model(
	init: PluginFactoryInit,
	index: &DynamicIndex,
	sender: AsyncFactorySender<PluginFactoryModel>,
) -> anyhow::Result<PluginFactoryModel> {
	let index = index.current_index();
	let plugin = init.plugin.clone();

	let (tx, rx) = relm4::tokio::sync::mpsc::channel(100);
	if plugin.start().await.is_ok() {
		let mut client = init.plugin.connect().await?;
		let mut stream = client.read_all_lists(Empty {}).await?.into_inner();
		relm4::spawn(async move {
			while let Some(list) = stream.message().await.unwrap() {
				tx.send(list).await.unwrap()
			}
		});
	}

	Ok(PluginFactoryModel {
		plugin,
		enabled: init.enabled,
		last_list_selected: None,
		list_factory: AsyncFactoryVecDeque::new(
			adw::ExpanderRow::default(),
			sender.input_sender(),
		),
		new_list_controller: ListEntryModel::builder().launch(()).forward(
			sender.input_sender(),
			move |message| match message {
				ListEntryOutput::AddTaskListToSidebar(name) => {
					PluginFactoryInput::RequestAddList(index, name)
				},
			},
		),
		rx,
	})
}
