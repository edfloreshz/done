use crate::widgets::content::messages::TaskInput;
use crate::widgets::content::messages::{ContentInput, ContentOutput};
use crate::widgets::preferences::model::Preferences;
use crate::widgets::smart_lists::widget::{
	SmartListContainerInit, SmartListContainerInput, SmartListContainerModel,
};
use crate::widgets::task_entry::messages::{TaskEntryInput, TaskEntryOutput};
use crate::widgets::task_entry::model::TaskEntryModel;
use libset::format::FileFormat;
use libset::project::Project;

use relm4::component::{
	AsyncComponent, AsyncComponentController, AsyncComponentParts,
	AsyncComponentSender,
};
use relm4::factory::AsyncFactoryVecDeque;
use relm4::{
	adw, gtk,
	gtk::prelude::{BoxExt, OrientableExt, WidgetExt},
};
use relm4::{Component, ComponentController, RelmWidgetExt};

use crate::widgets::content::model::ContentModel;

use super::helpers::{
	add_task, hide_flap, remove_task, reveal_task_details, select_task_list,
	update_task,
};

#[relm4::component(pub async)]
impl AsyncComponent for ContentModel {
	type CommandOutput = ();
	type Input = ContentInput;
	type Output = ContentOutput;
	type Init = ();

	view! {
		#[root]
		gtk::Stack {
			set_vexpand: true,
			set_transition_duration: 250,
			set_transition_type: gtk::StackTransitionType::Crossfade,
			set_width_request: 400,
			#[name(flap)]
			adw::Flap {
				set_modal: true,
				set_locked: true,
				#[watch]
				set_reveal_flap: model.show_task_details,
				#[wrap(Some)]
				set_content = &gtk::Box {
					set_width_request: 300,
					set_margin_all: 10,
					set_orientation: gtk::Orientation::Vertical,
					#[name(task_container)]
					gtk::Stack {
						set_transition_duration: 250,
						set_transition_type: gtk::StackTransitionType::Crossfade,
						gtk::ScrolledWindow {
							#[watch]
							set_visible: model.parent_list.is_some(),
							set_vexpand: true,
							set_hexpand: true,
							set_child: Some(list_box),
						},
						gtk::ScrolledWindow {
							#[watch]
							set_visible: model.selected_smart_list.is_some(),
							set_vexpand: true,
							set_hexpand: true,
							set_child: Some(model.smart_lists.widget())
						},
					},
					append: model.task_entry.widget()
				},
				#[wrap(Some)]
				#[local_ref]
				set_flap = flap_container -> gtk::Box {
					set_width_request: 300,
					set_css_classes: &["background"],

				},
				#[wrap(Some)]
				set_separator = &gtk::Separator {
					set_orientation: gtk::Orientation::Vertical,
				},
				set_flap_position: gtk::PackType::End,
			}
		}
	}

	async fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: AsyncComponentSender<Self>,
	) -> AsyncComponentParts<Self> {
		let compact = Project::open("dev", "edfloreshz", "done")
			.unwrap()
			.get_file_as::<Preferences>("preferences", FileFormat::JSON)
			.unwrap()
			.compact;
		let smart_lists = SmartListContainerModel::builder()
			.launch(SmartListContainerInit::new(None))
			.forward(sender.input_sender(), |message| match message {});

		let model = ContentModel {
			task_factory: AsyncFactoryVecDeque::new(
				gtk::ListBox::builder()
					.show_separators(true)
					.css_classes(vec!["boxed-list".to_string()])
					.valign(gtk::Align::Start)
					.margin_top(5)
					.margin_bottom(5)
					.margin_start(5)
					.margin_end(5)
					.build(),
				sender.input_sender(),
			),
			task_details_factory: AsyncFactoryVecDeque::new(
				gtk::Box::default(),
				sender.input_sender(),
			),
			task_entry: TaskEntryModel::builder().launch(None).forward(
				sender.input_sender(),
				|message| match message {
					TaskEntryOutput::EnterCreationMode(task) => {
						ContentInput::RevealTaskDetails(None, task)
					},
					TaskEntryOutput::AddTask(task) => ContentInput::AddTask(task),
				},
			),
			smart_lists,
			plugin: None,
			parent_list: None,
			selected_smart_list: None,
			compact,
			selected_task: None,
			show_task_details: false,
		};
		let list_box = model.task_factory.widget();
		let flap_container = model.task_details_factory.widget();

		let widgets = view_output!();

		AsyncComponentParts { model, widgets }
	}

	async fn update(
		&mut self,
		message: Self::Input,
		sender: AsyncComponentSender<Self>,
		_root: &Self::Root,
	) {
		match message {
			ContentInput::AddTask(mut task) => {
				if let Err(err) = add_task(self, sender, &mut task).await {
					tracing::error!("{err}");
				}
			},
			ContentInput::RemoveTask(index) => {
				if let Err(err) = remove_task(self, sender, index).await {
					tracing::error!("{err}");
				}
			},
			ContentInput::UpdateTask(task) => {
				if let Err(err) = update_task(self, sender, task).await {
					tracing::error!("{err}");
				}
			},
			ContentInput::TaskListSelected(list_model) => {
				if let Err(err) = select_task_list(self, list_model).await {
					tracing::error!("{err}");
				}
			},
			ContentInput::SelectSmartList(list) => {
				self.selected_smart_list = Some(list.clone());
				self
					.smart_lists
					.sender()
					.send(SmartListContainerInput::SetSmartList(list))
					.unwrap();
				self.parent_list = None;
			},
			ContentInput::RevealTaskDetails(index, task) => {
				reveal_task_details(self, index, task)
			},
			ContentInput::ToggleCompact(compact) => {
				let size = self.task_factory.len();
				for index in 0..size {
					self
						.task_factory
						.send(index, TaskInput::ToggleCompact(compact));
				}
			},
			ContentInput::DisablePlugin => self.parent_list = None,
			ContentInput::CleanTaskEntry => self
				.task_entry
				.sender()
				.send(TaskEntryInput::CleanTaskEntry)
				.unwrap(),
			ContentInput::HideFlap => hide_flap(self, sender),
		}
	}
}
