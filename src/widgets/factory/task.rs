use adw::traits::{EntryRowExt, PreferencesRowExt};
use proto_rust::provider::TaskStatus;
use proto_rust::provider_client::ProviderClient;
use proto_rust::Channel;
use relm4::factory::AsyncFactoryComponent;
use relm4::factory::{AsyncFactorySender, DynamicIndex, FactoryView};
use relm4::loading_widgets::LoadingWidgets;
use relm4::{
	gtk, adw,
	gtk::prelude::{
		BoxExt, ButtonExt, CheckButtonExt, EditableExt, EntryBufferExtManual,
		EntryExt, ListBoxRowExt, OrientableExt, ToggleButtonExt, WidgetExt,
	},
	RelmWidgetExt,
};

use crate::application::plugin::Plugin;
use crate::widgets::components::content::ContentComponentInput;
use proto_rust::provider::Task;

#[derive(Debug)]
pub enum TaskFactoryInput {
	SetCompleted(bool),
	Favorite(DynamicIndex),
	ModifyTitle(String),
	ToggleCompact(bool),
}

#[derive(Debug)]
pub enum TaskFactoryOutput {
	Remove(DynamicIndex),
	UpdateTask(Option<DynamicIndex>, Task),
}

#[derive(Debug, Clone)]
pub struct TaskFactoryModel {
	pub task: Task,
	pub client: ProviderClient<Channel>,
	pub compact: bool,
	pub first_load: bool,
}

#[derive(derive_new::new)]
pub struct TaskFactoryInit {
	plugin: Plugin,
	id: String,
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
			set_selectable: false,
			set_title: "Task name",
			set_text: self.task.title.as_str(),
			set_show_apply_button: true,
			set_enable_emoji_completion: true,
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
				connect_toggled[sender, index] => move |_| {
					sender.input(TaskFactoryInput::Favorite(index.clone()));
				}
			},
			#[name(delete)]
			add_suffix = &gtk::Button {
				add_css_class: "destructive-action",
				add_css_class: "circular",
				set_icon_name: "user-trash-full-symbolic",
				connect_clicked[sender, index] => move |_| {
					sender.output(TaskFactoryOutput::Remove(index.clone()))
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
		let mut model = Self {
			task: Task::default(),
			client: init.plugin.connect().await.unwrap(),
			compact: init.compact,
			first_load: true,
		};
		let mut client = model.client.clone();
		let id = init.id.clone();
		match relm4::spawn(async move { client.read_task(id).await })
			.await
			.unwrap()
		{
			Ok(response) => match response.into_inner().task {
				Some(task) => model.task = task,
				None => tracing::error!("Failed to get task."),
			},
			Err(e) => tracing::error!("Failed to find tasks. {:?}", e),
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
		})
	}
}
