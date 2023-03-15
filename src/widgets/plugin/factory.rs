use crate::widgets::sidebar::messages::SidebarComponentInput;
use crate::widgets::task_list::model::ListFactoryInit;
use adw::prelude::PreferencesRowExt;
use adw::traits::ExpanderRowExt;
use relm4::factory::AsyncFactoryComponent;
use relm4::factory::AsyncFactoryVecDeque;
use relm4::factory::{AsyncFactorySender, DynamicIndex, FactoryView};
use relm4::gtk;
use relm4::gtk::prelude::WidgetExt;
use relm4::ComponentController;
use relm4::{adw, Component};

use crate::widgets::list_entry::{ListEntryModel, ListEntryOutput};

use super::messages::{PluginFactoryInput, PluginFactoryOutput};
use super::model::{PluginFactoryInit, PluginFactoryModel};

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
				set_direction: gtk::ArrowType::Up,
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
			PluginFactoryInput::Enable => {
				self.enabled = true;
				sender.input(PluginFactoryInput::FillTaskFactory);
			},
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
	if init.enabled && plugin.start().await.is_ok() {
		let mut client = init.plugin.connect().await?;
		let mut stream = client.get_lists(()).await?.into_inner();
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
