use adw::traits::{EntryRowExt, PreferencesRowExt};
use done_provider::provider::Status;
use relm4::factory::AsyncFactoryComponent;
use relm4::factory::{AsyncFactorySender, DynamicIndex, FactoryView};
use relm4::{
	adw, gtk,
	gtk::prelude::{
		ButtonExt, CheckButtonExt, EditableExt, ListBoxRowExt, WidgetExt,
	},
	RelmWidgetExt,
};

use crate::widgets::content::messages::ContentInput;
use crate::widgets::content::messages::{TaskInput, TaskOutput};
use crate::widgets::content::model::{TaskInit, TaskModel};

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for TaskModel {
	type ParentInput = ContentInput;
	type ParentWidget = gtk::ListBox;
	type CommandOutput = ();
	type Input = TaskInput;
	type Output = TaskOutput;
	type Init = TaskInit;
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
					sender.input(TaskInput::SetCompleted(checkbox.is_active()));
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
					sender.input(TaskInput::Favorite(index.clone()));
				}
			},
			#[name(details)]
			add_suffix = &gtk::Button {
				add_css_class: "suggested-action",
				add_css_class: "circular",
				set_icon_name: "info-symbolic",
				set_valign: gtk::Align::Center,
				connect_clicked[sender, index] => move |_| {
					sender.input(TaskInput::RevealTaskDetails(Some(index.clone())))
				}
			},
			#[name(delete)]
			add_suffix = &gtk::Button {
				add_css_class: "destructive-action",
				add_css_class: "circular",
				set_icon_name: "user-trash-full-symbolic",
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
		_index: &DynamicIndex,
		_sender: AsyncFactorySender<Self>,
	) -> Self {
		Self {
			task: init.task,
			parent_list: init.parent_list,
			compact: init.compact,
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
			TaskInput::RevealTaskDetails(index) => {
				sender.output(TaskOutput::RevealTaskDetails(index, self.task.clone()))
			},
			TaskInput::SetCompleted(toggled) => {
				self.task.status = if toggled {
					Status::Completed as i32
				} else {
					Status::NotStarted as i32
				};
				sender
					.output_sender()
					.send(TaskOutput::UpdateTask(None, self.task.clone()))
					.unwrap_or_default();
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
			TaskInput::ToggleCompact(compact) => self.compact = compact,
		}
	}

	fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
		Some(match output {
			TaskOutput::Remove(index) => ContentInput::RemoveTask(index),
			TaskOutput::UpdateTask(_, task) => ContentInput::UpdateTask(task),
			TaskOutput::RevealTaskDetails(index, task) => {
				ContentInput::RevealTaskDetails(index, task)
			},
		})
	}
}
