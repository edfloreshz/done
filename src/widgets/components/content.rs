use crate::fl;
use crate::widgets::components::new_task::{
	NewTask, NewTaskEvent, NewTaskOutput,
};
use crate::widgets::factory::task::TaskData;
use done_provider::plugin::Plugin;
use done_provider::services::provider::provider_client::ProviderClient;
use done_provider::services::provider::{List, Task};
use done_provider::Channel;
use relm4::component::{
	AsyncComponent, AsyncComponentParts, AsyncComponentSender,
};
use relm4::factory::AsyncFactoryVecDeque;
use relm4::factory::DynamicIndex;
use relm4::{
	gtk,
	gtk::prelude::{BoxExt, OrientableExt, WidgetExt},
	view, Controller, RelmWidgetExt,
};
use relm4::{Component, ComponentController};
use std::str::FromStr;

pub struct ContentModel {
	current_provider: Plugin,
	parent_list: Option<List>,
	tasks_factory: AsyncFactoryVecDeque<TaskData>,
	new_task_component: Controller<NewTask>,
}

#[derive(Debug)]
pub enum ContentInput {
	AddTask(Task),
	RemoveTask(DynamicIndex),
	UpdateTask(Option<DynamicIndex>, Task),
	SetTaskList(List),
	SetProvider(Plugin),
}

#[derive(Debug)]
pub enum ContentOutput {
	Notify(String),
}

#[relm4::component(pub async)]
impl AsyncComponent for ContentModel {
	type Input = ContentInput;
	type Output = ContentOutput;
	type Init = Option<String>;
	type Widgets = ContentWidgets;
	type CommandOutput = ();

	view! {
		#[root]
		#[name(tasks)]
		gtk::Stack {
			set_vexpand: true,
			set_transition_duration: 250,
			set_transition_type: gtk::StackTransitionType::Crossfade,
			gtk::CenterBox {
				set_orientation: gtk::Orientation::Vertical,
				#[watch]
				set_visible: model.parent_list.is_none(),
				set_halign: gtk::Align::Center,
				set_valign: gtk::Align::Center,
				#[wrap(Some)]
				set_center_widget = &gtk::Box {
					set_orientation: gtk::Orientation::Vertical,
					set_margin_all: 24,
					set_spacing: 24,
					gtk::Picture {
						set_resource: Some("/dev/edfloreshz/Done/icons/scalable/actions/paper-plane.png"),
						set_margin_all: 70
					},
					gtk::Label {
						set_css_classes: &["title-2", "accent"],
						set_text: fl!("select-list")
					},
					gtk::Label {
						set_text: fl!("tasks-here")
					}
				}
			},
			gtk::Box {
				set_orientation: gtk::Orientation::Vertical,
				#[watch]
				set_visible: model.parent_list.is_some(),
				gtk::Box {
					#[name(task_container)]
					gtk::Stack {
						set_transition_duration: 250,
						set_transition_type: gtk::StackTransitionType::Crossfade,
						gtk::ScrolledWindow {
							set_vexpand: true,
							set_hexpand: true,
							set_child: Some(&list_box)
						},
					}
				},
				append: model.new_task_component.widget()
			},
		}
	}

	async fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: AsyncComponentSender<Self>,
	) -> AsyncComponentParts<Self> {
		view! {
			list_box = &gtk::Box {
				set_orientation: gtk::Orientation::Vertical,
			}
		}
		let model = ContentModel {
			current_provider: Plugin::Local,
			parent_list: None,
			tasks_factory: AsyncFactoryVecDeque::new(
				list_box.clone(),
				sender.input_sender(),
			),
			new_task_component: NewTask::builder().launch(None).forward(
				sender.input_sender(),
				|message| match message {
					NewTaskOutput::AddTask(task) => ContentInput::AddTask(task),
				},
			),
		};
		let widgets = view_output!();
		AsyncComponentParts { model, widgets }
	}

	async fn update_with_view(
		&mut self,
		widgets: &mut Self::Widgets,
		message: Self::Input,
		sender: AsyncComponentSender<Self>,
		_root: &Self::Root,
	) {
		let mut service: Option<ProviderClient<Channel>> = None;
		if let Some(parent) = &self.parent_list {
			if let Ok(provider) = Plugin::from_str(&parent.provider) {
				match provider.connect().await {
					Ok(connection) => service = Some(connection),
					Err(_) => {
						sender
							.output(ContentOutput::Notify(format!(
								"Failed to connect to {} service.",
								&parent.provider
							)))
							.unwrap_or_default();
					},
				}
			} else {
				sender
					.output(ContentOutput::Notify(format!(
						"Failed to find plugin with name: {}.",
						&parent.provider
					)))
					.unwrap_or_default();
			}
		}
		match message {
			ContentInput::AddTask(task) => {
				if let Some(mut service) = service {
					match service.create_task(task.clone()).await {
						Ok(response) => {
							let response = response.into_inner();
							if response.successful {
								self.tasks_factory.guard().push_back(TaskData {
									data: task,
									loaded: false,
								});
							}
							sender
								.output(ContentOutput::Notify(response.message))
								.unwrap_or_default();
						},
						Err(err) => {
							sender
								.output(ContentOutput::Notify(err.to_string()))
								.unwrap_or_default();
						},
					}
				}
			},
			ContentInput::RemoveTask(index) => {
				if let Some(mut service) = service {
					let mut guard = self.tasks_factory.guard();
					let task = guard.get(index.current_index()).unwrap();
					match service.delete_task(task.clone().data.id).await {
						Ok(response) => {
							let response = response.into_inner();
							if response.successful {
								guard.remove(index.current_index());
							}
							sender
								.output(ContentOutput::Notify(response.message))
								.unwrap_or_default();
						},
						Err(err) => {
							sender
								.output(ContentOutput::Notify(err.to_string()))
								.unwrap_or_default();
						},
					}
				}
			},
			ContentInput::UpdateTask(index, task) => {
				if let Some(mut service) = service {
					match service.update_task(task).await {
						Ok(response) => {
							let response = response.into_inner();
							if response.successful {
								if let Some(index) = index {
									if self.parent_list.as_ref().unwrap().provider == "starred" {
										self.tasks_factory.guard().remove(index.current_index());
									}
								}
							}
							sender
								.output(ContentOutput::Notify(response.message))
								.unwrap_or_default();
						},
						Err(err) => {
							sender
								.output(ContentOutput::Notify(err.to_string()))
								.unwrap_or_default();
						},
					}
				}
			},
			ContentInput::SetTaskList(list) => {
				self.parent_list = Some(list.clone());
				self
					.new_task_component
					.sender()
					.send(NewTaskEvent::SetParentList(self.parent_list.clone()))
					.unwrap_or_default();

				if let Ok(provider) = Plugin::from_str(&list.provider) {
					let mut service = provider.connect().await.unwrap();
					let (tx, mut rx) = 	tokio::sync::mpsc::channel(4);

					tokio::spawn(async move {
						let mut stream = service
							.read_tasks_from_list(list.id)
							.await
							.unwrap()
							.into_inner();
						while let Some(task) = stream.message().await.unwrap() {
							tx.send(task).await.unwrap()
						}
					});

					loop {
						let task = self.tasks_factory.guard().pop_front();
						if task.is_none() {
							break;
						}
					}

					while let Some(task) = rx.recv().await {
						if let Some(task) = task.task {
							self.tasks_factory.guard().push_back(TaskData {
								data: task,
								loaded: false,
							});
						}
					}
				} else {
					sender
						.output(ContentOutput::Notify(String::from(
							"Failed to identify the provider.",
						)))
						.unwrap_or_default();
				}
			},
			ContentInput::SetProvider(provider) => {
				self.current_provider = provider;
				self.parent_list = None;
				self
					.new_task_component
					.sender()
					.send(NewTaskEvent::SetParentList(None))
					.unwrap_or_default();
			},
		}
		self.update_view(widgets, sender)
	}
}
