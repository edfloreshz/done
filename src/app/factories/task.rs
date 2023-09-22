use crate::app::components::content::ContentInput;
use crate::fl;
use adw::prelude::{ActionableExt, ActionableExtManual};
use adw::traits::{EntryRowExt, PreferencesRowExt};
use core_done::models::list::List;
use core_done::models::status::Status;
use core_done::models::task::Task;
use relm4::factory::AsyncFactoryComponent;
use relm4::factory::{AsyncFactorySender, DynamicIndex, FactoryView};
use relm4::{
	adw, gtk,
	gtk::prelude::{
		ButtonExt, CheckButtonExt, EditableExt, ListBoxRowExt, WidgetExt,
	},
	RelmWidgetExt,
};
use relm4_icons::icon_name;

#[derive(Debug, Clone)]
pub struct TaskModel {
	pub task: Task,
	pub parent_list: List,
	pub index: DynamicIndex,
}

#[derive(derive_new::new)]
pub struct TaskInit {
	pub task: Task,
	pub parent_list: List,
}

#[derive(Debug)]
pub enum TaskInput {
	SetCompleted(bool),
	Favorite,
	ModifyTitle(String),
	RevealTaskDetails,
	SetTask(Task),
}

#[derive(Debug)]
pub enum TaskOutput {
	Remove(DynamicIndex),
	UpdateTask(DynamicIndex, Task),
	RevealTaskDetails(DynamicIndex, Task),
}

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for TaskModel {
	type ParentInput = ContentInput;
	type ParentWidget = adw::PreferencesGroup;
	type CommandOutput = ();
	type Input = TaskInput;
	type Output = TaskOutput;
	type Init = TaskInit;
	type Widgets = TaskWidgets;

	view! {
		root = adw::EntryRow {
			#[watch]
			set_title: &self.parent_list.name,
			#[watch]
			set_text: self.task.title.as_str(),
			set_show_apply_button: true,
			set_enable_emoji_completion: true,
			#[name(check_button)]
			add_prefix = &gtk::CheckButton {
				set_tooltip: fl!("completed-tooltip"),
				#[watch]
				set_active: self.task.status == Status::Completed,
				connect_toggled[sender] => move |checkbox| {
					sender.input(TaskInput::SetCompleted(checkbox.is_active()));
				}
			},
			#[name(favorite)]
			add_suffix = &gtk::ToggleButton {
				add_css_class: "opaque",
				add_css_class: "circular",
				#[watch]
				set_class_active: ("favorite", self.task.favorite),
				set_icon_name: icon_name::STAR_FILLED_ROUNDED,
				set_valign: gtk::Align::Center,
				set_tooltip: fl!("favorite-task"),
				connect_clicked => TaskInput::Favorite,
			},
			#[name(details)]
			add_suffix = &gtk::Button {
				add_css_class: "suggested-action",
				add_css_class: "circular",
				set_icon_name: icon_name::INFO,
				set_valign: gtk::Align::Center,
				set_tooltip: fl!("edit-task-details"),
				set_action_name: Some("navigation.push"),
				set_action_target: Some("task-details-page"),
				connect_clicked => TaskInput::RevealTaskDetails
			},
			#[name(delete)]
			add_suffix = &gtk::Button {
				add_css_class: "destructive-action",
				add_css_class: "circular",
				set_icon_name: icon_name::X_CIRCULAR,
				set_tooltip: fl!("remove-task"),
				set_valign: gtk::Align::Center,
				connect_clicked[sender, index] => move |_| {
					sender.output(TaskOutput::Remove(index.clone()))
				}
			},
			connect_activate[sender] => move |entry| {
				let buffer = entry.text().to_string();
				sender.input(TaskInput::ModifyTitle(buffer));
			},
			connect_apply[sender] => move |entry| {
				let buffer = entry.text().to_string();
				sender.input(TaskInput::ModifyTitle(buffer));
			},
		}
	}

	async fn init_model(
		init: Self::Init,
		index: &DynamicIndex,
		_sender: AsyncFactorySender<Self>,
	) -> Self {
		let mut task = init.task.clone();
		task.parent = init.parent_list.id.clone();
		Self {
			task,
			parent_list: init.parent_list,
			index: index.clone(),
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
			TaskInput::SetTask(task) => self.task = task,
			TaskInput::RevealTaskDetails => sender.output(
				TaskOutput::RevealTaskDetails(self.index.clone(), self.task.clone()),
			),
			TaskInput::SetCompleted(toggled) => {
				self.task.status = if toggled {
					Status::Completed
				} else {
					Status::NotStarted
				};
				sender
					.output_sender()
					.send(TaskOutput::UpdateTask(
						self.index.clone(),
						self.task.clone(),
					))
					.unwrap_or_default();
			},
			TaskInput::Favorite => {
				self.task.favorite = !self.task.favorite;

				sender
					.output_sender()
					.send(TaskOutput::UpdateTask(
						self.index.clone(),
						self.task.clone(),
					))
					.unwrap_or_default();
			},
			TaskInput::ModifyTitle(title) => {
				if title != self.task.title {
					self.task.title = title;
					sender
						.output_sender()
						.send(TaskOutput::UpdateTask(
							self.index.clone(),
							self.task.clone(),
						))
						.unwrap_or_default();
				}
			},
		}
	}

	fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
		Some(match output {
			TaskOutput::Remove(index) => ContentInput::RemoveTask(index),
			TaskOutput::UpdateTask(index, task) => {
				ContentInput::UpdateTask(Some(index), task)
			},
			TaskOutput::RevealTaskDetails(index, task) => {
				ContentInput::RevealTaskDetails(Some(index), task)
			},
		})
	}
}
