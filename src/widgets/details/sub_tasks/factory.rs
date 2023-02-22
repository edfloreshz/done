use adw::traits::{EntryRowExt, PreferencesRowExt};
use gtk::traits::{ButtonExt, CheckButtonExt, ListBoxRowExt, WidgetExt};
use relm4::gtk::traits::EditableExt;
use relm4::{
	adw,
	factory::FactoryView,
	gtk,
	prelude::{DynamicIndex, FactoryComponent},
	FactorySender,
};

use crate::widgets::details::messages::TaskDetailsFactoryInput;

use super::{
	messages::{SubTaskInput, SubTaskOutput},
	model::{SubTaskInit, SubTaskModel},
};

#[relm4::factory(pub)]
impl FactoryComponent for SubTaskModel {
	type ParentWidget = adw::PreferencesGroup;
	type ParentInput = TaskDetailsFactoryInput;
	type Input = SubTaskInput;
	type Output = SubTaskOutput;
	type Init = SubTaskInit;
	type CommandOutput = ();

	view! {
		#[root]
		adw::EntryRow {
			#[watch]
			set_title: "Sub-task",
			set_enable_emoji_completion: true,
			set_show_apply_button: true,
			set_text: self.sub_task.title.as_str(),
			add_prefix = &gtk::CheckButton {
				set_active: self.sub_task.completed,
				connect_toggled[sender, index] => move |checkbox| {
					sender.input(SubTaskInput::SetCompleted(index.clone(), checkbox.is_active()));
				}
			},
			add_suffix = &gtk::Button {
				set_valign: gtk::Align::Center,
				set_icon_name: "user-trash-full-symbolic",
				set_css_classes: &["destructive-action", "circular"],
				connect_clicked[sender, index] => move |_| {
					sender.input(SubTaskInput::Remove(index.clone()));
				}
			},
			connect_activate[sender, index] => move |entry| {
				let buffer = entry.text().to_string();
				sender.input(SubTaskInput::ModifyTitle(index.clone(), buffer));
			},
			connect_apply[sender, index] => move |entry| {
				let buffer = entry.text().to_string();
				sender.input(SubTaskInput::ModifyTitle(index.clone(), buffer));
			},
		}
	}

	fn init_model(
		init: Self::Init,
		index: &DynamicIndex,
		_sender: FactorySender<Self>,
	) -> Self {
		Self {
			sub_task: init.sub_task,
			index: index.clone(),
		}
	}

	fn init_widgets(
		&mut self,
		index: &DynamicIndex,
		root: &Self::Root,
		_returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
		sender: FactorySender<Self>,
	) -> Self::Widgets {
		let widgets = view_output!();
		widgets
	}

	fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
		match message {
			SubTaskInput::SetCompleted(index, completed) => {
				self.sub_task.completed = completed;
				sender.output(SubTaskOutput::Update(index, self.sub_task.clone()))
			},
			SubTaskInput::ModifyTitle(index, title) => {
				self.sub_task.title = title;
				sender.output(SubTaskOutput::Update(index, self.sub_task.clone()))
			},
			SubTaskInput::Remove(index) => {
				sender.output(SubTaskOutput::Remove(index))
			},
		}
	}

	fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
		let output = match output {
			SubTaskOutput::Update(index, sub_task) => {
				TaskDetailsFactoryInput::UpdateSubTask(index, sub_task)
			},
			SubTaskOutput::Remove(index) => {
				TaskDetailsFactoryInput::RemoveSubTask(index)
			},
		};
		Some(output)
	}
}
