use adw::traits::{EntryRowExt, PreferencesRowExt};
use proto_rust::provider::TaskStatus;
use proto_rust::List;
use relm4::factory::AsyncFactoryComponent;
use relm4::factory::{AsyncFactorySender, DynamicIndex, FactoryView};
use relm4::{
	adw, gtk,
	gtk::prelude::{
		ButtonExt, CheckButtonExt, EditableExt, ListBoxRowExt, WidgetExt,
	},
	RelmWidgetExt,
};

use crate::widgets::components::content::ContentComponentInput;
use proto_rust::provider::Task;

#[derive(Debug)]
pub enum TaskFactoryInput {
	SetCompleted(bool),
	Favorite(DynamicIndex),
	ModifyTitle(String),
	ToggleCompact(bool),
	RevealTaskDetails(DynamicIndex),
}

#[derive(Debug)]
pub enum TaskFactoryOutput {
	Remove(DynamicIndex),
	UpdateTask(Option<DynamicIndex>, Task),
	RevealTaskDetails(DynamicIndex, Task),
}

#[derive(Debug, Clone)]
pub struct TaskFactoryModel {
	pub task: Task,
	pub parent_list: List,
	pub compact: bool,
	pub first_load: bool,
}

#[derive(derive_new::new)]
pub struct TaskFactoryInit {
	task: Task,
	parent_list: List,
	compact: bool,
}

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for TaskFactoryModel {
	type ParentInput = ContentComponentInput;
	type ParentWidget = gtk::ListBox;
	type CommandOutput = ();
	type Input = TaskFactoryInput;
	type Output = TaskFactoryOutput;
	type Init = TaskFactoryInit;
	type Widgets = TaskWidgets;

	view! {
		root = adw::EntryRow {
			set_title: self.parent_list.name.as_str(),
			set_text: self.task.title.as_str(),
			set_show_apply_button: true,
			set_enable_emoji_completion: true,
			#[watch]
			set_margin_all: if self.compact {
				0
			} else {
				2
			},
			#[name(check_button)]
			add_prefix = &gtk::CheckButton {
				set_active: self.task.status == 1,
				connect_toggled[sender] => move |checkbox| {
					sender.input(TaskFactoryInput::SetCompleted(checkbox.is_active()));
				}
			},
			#[name(favorite)]
			add_suffix = &gtk::ToggleButton {
				add_css_class: "opaque",
				add_css_class: "circular",
				#[watch]
				set_class_active: ("favorite", self.task.favorite),
				set_icon_name: "star-filled-rounded-symbolic",
				set_valign: gtk::Align::Center,
				connect_clicked[sender, index] => move |_| {
					sender.input(TaskFactoryInput::Favorite(index.clone()));
				}
			},
			#[name(delete)]
			add_suffix = &gtk::Button {
				add_css_class: "destructive-action",
				add_css_class: "circular",
				set_icon_name: "user-trash-full-symbolic",
				set_valign: gtk::Align::Center,
				connect_clicked[sender, index] => move |_| {
					sender.output(TaskFactoryOutput::Remove(index.clone()))
				}
			},
			#[name(details)]
			add_suffix = &gtk::Button {
				add_css_class: "suggested-action",
				add_css_class: "circular",
				set_icon_name: "info-symbolic",
				set_valign: gtk::Align::Center,
				connect_clicked[sender, index] => move |_| {
					sender.input(TaskFactoryInput::RevealTaskDetails(index.clone()))
				}
			},
			connect_activate[sender] => move |entry| {
				let buffer = entry.text().to_string();
				sender.input(TaskFactoryInput::ModifyTitle(buffer));
			},
			connect_apply[sender] => move |entry| {
				let buffer = entry.text().to_string();
				sender.input(TaskFactoryInput::ModifyTitle(buffer));
			},
		}
	}

	async fn init_model(
		init: Self::Init,
		_index: &DynamicIndex,
		_sender: AsyncFactorySender<Self>,
	) -> Self {
		Self {
			task: init.task,
			parent_list: init.parent_list,
			compact: init.compact,
			first_load: true,
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
		widgets
	}

	async fn update(
		&mut self,
		message: Self::Input,
		sender: AsyncFactorySender<Self>,
	) {
		match message {
			TaskFactoryInput::RevealTaskDetails(index) => sender.output(
				TaskFactoryOutput::RevealTaskDetails(index, self.task.clone()),
			),
			TaskFactoryInput::SetCompleted(toggled) => {
				self.task.status = if toggled {
					TaskStatus::Completed as i32
				} else {
					TaskStatus::NotStarted as i32
				};
				if !self.first_load {
					sender
						.output_sender()
						.send(TaskFactoryOutput::UpdateTask(None, self.task.clone()))
						.unwrap_or_default();
				}
			},
			TaskFactoryInput::Favorite(index) => {
				self.task.favorite = !self.task.favorite;

				sender
					.output_sender()
					.send(TaskFactoryOutput::UpdateTask(
						Some(index),
						self.task.clone(),
					))
					.unwrap_or_default();
			},
			TaskFactoryInput::ModifyTitle(title) => {
				if title != self.task.title {
					self.task.title = title;
					sender
						.output_sender()
						.send(TaskFactoryOutput::UpdateTask(None, self.task.clone()))
						.unwrap_or_default();
				}
			},
			TaskFactoryInput::ToggleCompact(compact) => self.compact = compact,
		}
		self.first_load = false;
	}

	fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
		Some(match output {
			TaskFactoryOutput::Remove(index) => {
				ContentComponentInput::RemoveTask(index)
			},
			TaskFactoryOutput::UpdateTask(index, task) => {
				ContentComponentInput::UpdateTask(index, task)
			},
			TaskFactoryOutput::RevealTaskDetails(index, task) => {
				ContentComponentInput::RevealTaskDetails(index, task)
			},
		})
	}
}
