use crate::widgets::factory::task::TaskData;
use crate::{fl, rt};
use done_core::plugins::Plugin;
use done_core::provider::{List, ProviderRequest, Task};
use relm4::factory::{DynamicIndex, FactoryVecDeque};
use relm4::{
	gtk,
	gtk::prelude::{
		BoxExt, EntryBufferExtManual, EntryExt, OrientableExt, WidgetExt,
	},
	view, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent,
};
use std::str::FromStr;

pub struct ContentModel {
	current_provider: Plugin,
	parent_list: Option<List>,
	tasks_factory: FactoryVecDeque<TaskData>,
}

#[derive(Debug)]
pub enum ContentInput {
	AddTask(String),
	RemoveTask(DynamicIndex),
	SetTaskList(List),
	SetProvider(Plugin),
	UpdateTask(Option<DynamicIndex>, Task),
}

#[derive(Debug)]
pub enum ContentOutput {}

#[relm4::component(pub)]
impl SimpleComponent for ContentModel {
	type Input = ContentInput;
	type Output = ContentOutput;
	type Init = Option<Task>;
	type Widgets = ContentWidgets;

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
					gtk::Picture::for_resource("/dev/edfloreshz/Done/icons/scalable/actions/all-done.svg"),
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
				gtk::Box {
					set_orientation: gtk::Orientation::Horizontal,
					set_margin_all: 12,
					#[name(entry)]
					gtk::Entry {
						set_hexpand: true,
						#[watch]
						set_visible: model.parent_list.is_some(),
						set_icon_from_icon_name: (gtk::EntryIconPosition::Primary, Some("value-increase-symbolic")),
						set_placeholder_text: Some(fl!("new-task")),
						set_height_request: 42,
						connect_activate[sender] => move |entry| {
							let buffer = entry.buffer();
							sender.input(ContentInput::AddTask(buffer.text()));
							buffer.delete_text(0, None);
						}
					}
				}
			},
		}
	}

	fn init(
		_init: Self::Init,
		root: &Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		view! {
			list_box = &gtk::Box {
					set_orientation: gtk::Orientation::Vertical,
			}
		}
		let model = ContentModel {
			current_provider: Plugin::Local,
			parent_list: None,
			tasks_factory: FactoryVecDeque::new(
				list_box.clone(),
				&sender.input_sender(),
			),
		};
		let widgets = view_output!();
		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		let parent_list = &self.parent_list;
		if let Some(parent) = parent_list {
			if let Ok(provider) = Plugin::from_str(&parent.provider) {
				let mut service = rt().block_on(provider.connect()).unwrap();
				match message {
					ContentInput::AddTask(title) => {
						let task = Task::new(title, parent.id.to_owned());
						let response = rt()
							.block_on(service.create_task(ProviderRequest {
								list: parent_list.clone(),
								task: Some(task.clone()),
							}))
							.unwrap();

						if response.into_inner().successful {
							self
								.tasks_factory
								.guard()
								.push_back(TaskData { data: task });
						}
					},
					ContentInput::RemoveTask(index) => {
						let mut guard = self.tasks_factory.guard();
						let task = guard.get(index.current_index()).unwrap();
						let response = rt()
							.block_on(service.delete_task(ProviderRequest {
								list: parent_list.clone(),
								task: Some(task.data.clone()),
							}))
							.unwrap();

						if response.into_inner().successful {
							guard.remove(index.current_index());
						}
					},
					ContentInput::UpdateTask(index, task) => {
						let response = rt()
							.block_on(service.update_task(ProviderRequest {
								list: parent_list.clone(),
								task: Some(task.clone()),
							}))
							.unwrap();

						if response.into_inner().successful {
							if let Some(index) = index {
								if self.parent_list.as_ref().unwrap().provider == "starred" {
									self.tasks_factory.guard().remove(index.current_index());
								}
							}
						}
					},
					_ => {},
				}
			} else {
				todo!("Display connection error")
			}
		} else {
			match message {
				ContentInput::SetProvider(provider) => {
					self.current_provider = provider;
					self.parent_list = None;
				},
				ContentInput::SetTaskList(list) => {
					self.parent_list = Some(list.clone());
					if let Ok(provider) = Plugin::from_str(&list.provider) {
						let mut service = rt().block_on(provider.connect()).unwrap();

						let response = rt()
							.block_on(service.read_tasks_from_list(ProviderRequest {
								list: Some(list.clone()),
								task: None,
							}))
							.unwrap()
							.into_inner();

						let mut tasks: Vec<Task> = vec![];
						if response.successful {
							tasks =
								serde_json::from_str(response.data.unwrap().as_str()).unwrap();
						}

						loop {
							let task = self.tasks_factory.guard().pop_front();
							if task.is_none() {
								break;
							}
						}
						for task in tasks {
							self
								.tasks_factory
								.guard()
								.push_back(TaskData { data: task });
						}
					} else {
						todo!("Display connection error")
					}
				},
				_ => {},
			}
		}
	}
}
