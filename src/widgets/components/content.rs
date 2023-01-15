use crate::application::plugin::Plugin;
use crate::widgets::components::new_task::{
	NewTask, NewTaskEvent, NewTaskOutput,
};
use crate::widgets::factory::list::ListData;
use crate::widgets::factory::task::TaskData;
use proto_rust::provider::provider_client::ProviderClient;
use proto_rust::provider::{List, Task};

use relm4::component::{
	AsyncComponent, AsyncComponentParts, AsyncComponentSender,
};
use relm4::factory::AsyncFactoryVecDeque;
use relm4::factory::DynamicIndex;
use relm4::{
	gtk,
	gtk::prelude::{BoxExt, OrientableExt, WidgetExt},
	view, Controller,
};
use relm4::{Component, ComponentController};
use std::str::FromStr;
use tonic::transport::Channel;

pub struct ContentModel {
	parent_list: Option<List>,
	task_factory: AsyncFactoryVecDeque<TaskData>,
	create_task_controller: Controller<NewTask>,
}

#[derive(Debug)]
pub enum ContentInput {
	AddTask(Task),
	RemoveTask(DynamicIndex),
	UpdateTask(Option<DynamicIndex>, Task),
	TaskListSelected(ListData),
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
		gtk::Box {
			#[name(tasks)]
			gtk::Stack {
				set_vexpand: true,
				set_transition_duration: 250,
				set_transition_type: gtk::StackTransitionType::Crossfade,
				gtk::Box {
					set_orientation: gtk::Orientation::Vertical,
					#[name(task_container)]
					gtk::Stack {
						set_transition_duration: 250,
						set_transition_type: gtk::StackTransitionType::Crossfade,
						gtk::ScrolledWindow {
							set_vexpand: true,
							set_hexpand: true,
							set_child: Some(&list_box)
						},
					},
					append: model.create_task_controller.widget()
				},
			}
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
			parent_list: None,
			task_factory: AsyncFactoryVecDeque::new(
				list_box.clone(),
				sender.input_sender(),
			),
			create_task_controller: NewTask::builder().launch(None).forward(
				sender.input_sender(),
				|message| match message {
					NewTaskOutput::AddTask(task) => ContentInput::AddTask(task),
				},
			),
		};
		let widgets = view_output!();
		AsyncComponentParts { model, widgets }
	}

	async fn update(
		&mut self,
		message: Self::Input,
		sender: AsyncComponentSender<Self>,
		_root: &Self::Root,
	) {
		//TODO: The freezing issue is somewhere here...
		let mut service: Option<ProviderClient<Channel>> = None;
		let mut plugin = None;
		if let Some(parent) = &self.parent_list {
			if let Ok(provider) = Plugin::from_str(&parent.provider) {
				match provider.connect().await {
					Ok(connection) => {
						plugin = Some(provider.data().await.unwrap());
						service = Some(connection)
					},
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
							if response.successful && response.task.is_some() {
								let task = response.task.unwrap();
								self
									.task_factory
									.guard()
									.push_back((task.id, plugin.unwrap().id));
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
					let mut guard = self.task_factory.guard();
					let task = guard.get(index.current_index()).unwrap();
					match service.delete_task(task.clone().task.id).await {
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
										self.task_factory.guard().remove(index.current_index());
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
			ContentInput::TaskListSelected(list) => {
				self.parent_list = Some(list.data.clone());
				self
					.create_task_controller
					.sender()
					.send(NewTaskEvent::SetParentList(self.parent_list.clone()))
					.unwrap_or_default();

				loop {
					let list = self.task_factory.guard().pop_front();
					if list.is_none() {
						break;
					}
				}

				for task in list.tasks {
					self
						.task_factory
						.guard()
						.push_back((task, list.data.provider.clone()));
				}
			},
		}
	}
}
