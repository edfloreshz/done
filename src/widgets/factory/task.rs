use proto_rust::provider::TaskStatus;
use proto_rust::provider_client::ProviderClient;
use proto_rust::Channel;
use relm4::factory::AsyncFactoryComponent;
use relm4::factory::{AsyncFactorySender, DynamicIndex, FactoryView};
use relm4::loading_widgets::LoadingWidgets;
use relm4::{
	gtk,
	gtk::prelude::{
		BoxExt, ButtonExt, CheckButtonExt, EditableExt, EntryBufferExtManual,
		EntryExt, ListBoxRowExt, OrientableExt, ToggleButtonExt, WidgetExt,
	},
	RelmWidgetExt,
};

use crate::widgets::components::content::ContentInput;
use proto_rust::provider::Task;

#[derive(Debug)]
pub enum TaskInput {
	SetCompleted(bool),
	Favorite(DynamicIndex),
	ModifyTitle(String),
}

#[derive(Debug)]
pub enum TaskOutput {
	Remove(DynamicIndex),
	UpdateTask(Option<DynamicIndex>, Task),
}

#[derive(Debug, Clone)]
pub struct TaskData {
	pub task: Task,
	pub service: ProviderClient<Channel>,
	pub first_load: bool,
}

#[derive(derive_new::new)]
pub struct TaskInit {
	id: String,
	service: ProviderClient<Channel>,
}

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for TaskData {
	type ParentInput = ContentInput;
	type ParentWidget = gtk::Box;
	type CommandOutput = ();
	type Input = TaskInput;
	type Output = TaskOutput;
	type Init = TaskInit;
	type Widgets = TaskWidgets;

	view! {
		root = gtk::ListBoxRow {
			set_selectable: false,
			#[name(container)]
			gtk::Box {
				set_orientation: gtk::Orientation::Horizontal,
				set_spacing: 5,
				set_margin_all: 10,
				#[name(check_button)]
				gtk::CheckButton {
					set_active: self.task.status == 1,
					connect_toggled[sender] => move |checkbox| {
						sender.input(TaskInput::SetCompleted(checkbox.is_active()));
					}
				},
				gtk::Box {
					set_orientation: gtk::Orientation::Horizontal,
					set_spacing: 15,
					#[name(entry)]
					gtk::Entry {
						add_css_class: "flat",
						add_css_class: "no-border",
						set_hexpand: true,
						set_text: &self.task.title,
						connect_activate[sender] => move |entry| {
							let buffer = entry.buffer();
							sender.input(TaskInput::ModifyTitle(buffer.text()));
						},
						connect_changed[sender] => move |entry| {
							let buffer = entry.buffer();
							sender.input(TaskInput::ModifyTitle(buffer.text()));
						},
					},
					#[name(favorite)]
					gtk::ToggleButton {
						add_css_class: "opaque",
						add_css_class: "circular",
						#[watch]
						set_class_active: ("favorite", self.task.favorite),
						set_icon_name: "star-filled-rounded-symbolic",
						connect_toggled[sender, index] => move |_| {
							sender.input(TaskInput::Favorite(index.clone()));
						}
					},
					#[name(delete)]
					gtk::Button {
						add_css_class: "destructive-action",
						add_css_class: "circular",
						set_icon_name: "user-trash-full-symbolic",
						connect_clicked[sender, index] => move |_| {
							sender.output(TaskOutput::Remove(index.clone()))
						}
					}
				}
			}
		}
	}

	fn init_loading_widgets(root: &mut Self::Root) -> Option<LoadingWidgets> {
		relm4::view! {
			#[local_ref]
			root {
				#[name(spinner)]
				gtk::Box {
					set_halign: gtk::Align::Center,
					set_valign: gtk::Align::Center,
					set_hexpand: true,
					set_vexpand: true,
					set_margin_all: 10,
					gtk::Spinner {
						start: (),
						set_hexpand: false,
					}
				}
			}
		}
		Some(LoadingWidgets::new(root, spinner))
	}

	async fn init_model(
		init: Self::Init,
		_index: &DynamicIndex,
		_sender: AsyncFactorySender<Self>,
	) -> Self {
		let mut model = Self {
			task: Task::default(),
			service: init.service,
			first_load: true,
		};
		match model.service.read_task(init.id.clone()).await {
			Ok(response) => match response.into_inner().task {
				Some(task) => model.task = task,
				None => error!("Failed to get task.")
			},
			Err(e) => error!("Failed to find tasks. {:?}", e),
		}
		model
	}

	fn init_widgets(
		&mut self,
		index: &DynamicIndex,
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
			TaskInput::SetCompleted(toggled) => {
				self.task.status = if toggled {
					TaskStatus::Completed as i32
				} else {
					TaskStatus::NotStarted as i32
				};
				if !self.first_load {
					sender
						.output_sender()
						.send(TaskOutput::UpdateTask(None, self.task.clone()))
						.unwrap_or_default();
				}
			},
			TaskInput::Favorite(index) => {
				self.task.favorite = !self.task.favorite;

				sender
					.output_sender()
					.send(TaskOutput::UpdateTask(Some(index), self.task.clone()))
					.unwrap_or_default();
			},
			TaskInput::ModifyTitle(title) => {
				if title != self.task.title {
					self.task.title = title;
					sender
						.output_sender()
						.send(TaskOutput::UpdateTask(None, self.task.clone()))
						.unwrap_or_default();
				}
			},
		}
		self.first_load = false;
	}

	fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
		Some(match output {
			TaskOutput::Remove(index) => ContentInput::RemoveTask(index),
			TaskOutput::UpdateTask(index, task) => {
				ContentInput::UpdateTask(index, task)
			},
		})
	}
}
