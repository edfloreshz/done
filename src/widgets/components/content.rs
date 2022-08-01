use relm4::factory::{DynamicIndex, FactoryVecDeque};
use relm4::{
	gtk,
	gtk::prelude::{
		BoxExt, ButtonExt, EntryBufferExtManual, EntryExt, OrientableExt, WidgetExt,
	},
	view, ComponentParts, ComponentSender, SimpleComponent, WidgetPlus,
};

use crate::data::models::generic::lists::GenericList;
use crate::data::models::generic::tasks::GenericTask;
use crate::{fl, Provider, SERVICES};
use crate::widgets::factory::list::ListType;
use crate::widgets::factory::list::ListType::{All, Other, Starred};

pub struct ContentModel {
	parent_list: Option<GenericList>,
	tasks: FactoryVecDeque<GenericTask>,
	show_tasks: bool,
}

#[derive(Debug)]
pub enum ContentInput {
	AddTask(String),
	RemoveTask(DynamicIndex),
	RemoveWelcomeScreen,
	SetTaskList(GenericList),
	UpdateCounters(Vec<ListType>),
	FavoriteTask(DynamicIndex, bool),
}

#[derive(Debug)]
pub enum ContentOutput {
	UpdateCounters(Vec<ListType>),
}

#[relm4::component(pub)]
impl SimpleComponent for ContentModel {
	type Input = ContentInput;
	type Output = ContentOutput;
	type InitParams = Option<GenericTask>;
	type Widgets = ContentWidgets;

	view! {
		#[root]
		tasks = &gtk::Stack {
			set_vexpand: true,
			set_transition_duration: 250,
			set_transition_type: gtk::StackTransitionType::Crossfade,
			add_child = &gtk::CenterBox {
				set_orientation: gtk::Orientation::Vertical,
				#[watch]
				set_visible: !model.show_tasks,
				set_halign: gtk::Align::Center,
				set_valign: gtk::Align::Center,
				#[wrap(Some)]
				set_center_widget = &gtk::Box {
					set_orientation: gtk::Orientation::Vertical,
					set_margin_all: 24,
					set_spacing: 24,
					append = &gtk::Picture::for_resource("/dev/edfloreshz/Done/icons/scalable/actions/all-done.svg"),
					append = &gtk::Label {
						add_css_class: "title",
						set_text: fl!("tasks-here")
					},
					append = &gtk::Button {
						#[watch]
						// set_visible: model.parent_list.0 > 5,
						add_css_class: "suggested-action",
						set_label: fl!("add-tasks"),
						connect_clicked[sender] => move |_| {
								sender.input(ContentInput::RemoveWelcomeScreen)
						}
					}
				}
			},
			add_child = &gtk::Box {
				set_orientation: gtk::Orientation::Vertical,
				#[watch]
				set_visible: model.show_tasks,
				append = &gtk::Box {
					append: task_container = &gtk::Stack {
						set_transition_duration: 250,
						set_transition_type: gtk::StackTransitionType::Crossfade,
						add_child = &gtk::ScrolledWindow {
							set_vexpand: true,
							set_hexpand: true,
							set_child: Some(&list_box)
						},
					}
				},
				append = &gtk::Box {
					set_orientation: gtk::Orientation::Horizontal,
					set_margin_all: 12,
					append: entry = &gtk::Entry {
						set_hexpand: true,
						#[watch]
						// set_visible: model.parent_list.0 > 5,
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
		_params: Self::InitParams,
		root: &Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		view! {
				list_box = &gtk::Box {
						set_orientation: gtk::Orientation::Vertical,
				}
		}
		let model = ContentModel {
			parent_list: None,
			tasks: FactoryVecDeque::new(list_box.clone(), &sender.input),
			show_tasks: false,
		};
		let widgets = view_output!();
		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
		let services = unsafe {
			&*SERVICES.get_mut().unwrap()
		};
		match message {
			ContentInput::AddTask(title) => {
				let parent_list = self.parent_list.as_ref().unwrap();
				let service = services.iter().find(|l| l.get_id() == parent_list.provider).unwrap();
				let task = GenericTask::new(title, parent_list.id_list.to_owned());
				let task = service.create_task(self.parent_list.as_ref().unwrap(), task).expect("Failed to post task.");
				self.tasks.guard().push_back(task);
			},
			ContentInput::RemoveTask(index) => {
				if self.tasks.guard().get(index.current_index()).unwrap().favorite {
					// sender.output(ContentOutput::UpdateCounters(vec![
					// 	All(-1),
					// 	Starred(-1),
					// 	Other(self.parent_list.0, -1),
					// ]));
				} else {
					// sender.output(ContentOutput::UpdateCounters(vec![
					// 	All(-1),
					// 	Other(self.parent_list.0, -1),
					// ]));
				}
				{
					let _task = self.tasks.guard().get(index.current_index());
					// delete_task(&task.id_task).expect("Failed to remove task.");
				}
				self.tasks.guard().remove(index.current_index());
			},
			ContentInput::RemoveWelcomeScreen => self.show_tasks = true,
			ContentInput::SetTaskList(list) => {
				self.parent_list = Some(list.clone());
				let service = services.iter().find(|l| l.get_id() == list.provider).unwrap();
				let tasks: Vec<GenericTask> = service.read_tasks_from_list(&list.id_list).unwrap();
				loop {
					let task = self.tasks.guard().pop_front();
					if task.is_none() {
						break;
					}
				}
				for task in tasks {
					self.tasks.guard().push_back(task.clone());
				}
				self.show_tasks = !self.tasks.guard().is_empty();
			},
			ContentInput::UpdateCounters(lists) => {
				sender.output(ContentOutput::UpdateCounters(lists))
			},
			ContentInput::FavoriteTask(index, favorite) => {
				// if self.parent_list.0 == 4 {
				// 	guard.remove(index.current_index());
				// }
				let value = if favorite { 1 } else { -1 };
				sender.output(ContentOutput::UpdateCounters(vec![Starred(value)]))
			},
		}
	}
}
