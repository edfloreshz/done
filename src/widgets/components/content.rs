use relm4::factory::{DynamicIndex, FactoryVecDeque};
use relm4::{
	gtk,
	gtk::prelude::{
		BoxExt, ButtonExt, EntryBufferExtManual, EntryExt, OrientableExt, WidgetExt,
	},
	view, ComponentParts, ComponentSender, SimpleComponent, WidgetPlus,
};

use crate::data::models::generic::lists::GenericTaskList;
use crate::data::models::generic::tasks::GenericTask;
use crate::data::plugins::all::AllProvider;
use crate::{fl, Provider, PLUGINS};

pub struct ContentModel {
	parent_list: GenericTaskList,
	tasks_factory: FactoryVecDeque<GenericTask>,
	show_tasks: bool,
}

#[derive(Debug)]
pub enum ContentInput {
	AddTask(String),
	RemoveTask(DynamicIndex),
	RemoveWelcomeScreen,
	SetTaskList(GenericTaskList),
	UpdateTask(Option<DynamicIndex>, GenericTask),
}

#[derive(Debug)]
pub enum ContentOutput {}

#[relm4::component(pub)]
impl SimpleComponent for ContentModel {
	type Input = ContentInput;
	type Output = ContentOutput;
	type Init = Option<GenericTask>;
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
				set_visible: !model.show_tasks,
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
					},
					gtk::Button {
						#[watch]
						set_visible: !model.parent_list.is_smart,
						add_css_class: "suggested-action",
						set_label: fl!("add-tasks"),
						connect_clicked[sender] => move |_| {
								sender.input(ContentInput::RemoveWelcomeScreen)
						}
					}
				}
			},
			gtk::Box {
				set_orientation: gtk::Orientation::Vertical,
				#[watch]
				set_visible: model.show_tasks,
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
						set_visible: !model.parent_list.is_smart,
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
		let all = AllProvider::new();
		let mut list = GenericTaskList::new(
			all.get_name(),
			all.get_icon_name(),
			0,
			all.get_id(),
		);
		list.make_smart();
		let model = ContentModel {
			parent_list: list.clone(),
			tasks_factory: FactoryVecDeque::new(list_box.clone(), &sender.input),
			show_tasks: false,
		};
		sender.input.send(ContentInput::SetTaskList(list));
		let widgets = view_output!();
		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		let parent_list = &self.parent_list;
		let service = PLUGINS.get_provider(&parent_list.provider);
		match message {
			ContentInput::AddTask(title) => {
				let task = GenericTask::new(title, parent_list.id_list.to_owned());
				let task = service
					.create_task(&self.parent_list, task)
					.expect("Failed to post task.");
				self.tasks_factory.guard().push_back(task);
			},
			ContentInput::RemoveTask(index) => {
				let mut guard = self.tasks_factory.guard();
				let task = guard.get(index.current_index()).unwrap();
				service
					.remove_task(&task.id_task)
					.expect("Failed to remove task.");
				guard.remove(index.current_index());
			},
			ContentInput::RemoveWelcomeScreen => self.show_tasks = true,
			ContentInput::SetTaskList(list) => {
				self.parent_list = list.clone();
				let service = PLUGINS.get_provider(&list.provider);
				let tasks: Vec<GenericTask> =
					service.read_tasks_from_list(&list.id_list).unwrap();
				loop {
					let task = self.tasks_factory.guard().pop_front();
					if task.is_none() {
						break;
					}
				}
				for task in tasks {
					self.tasks_factory.guard().push_back(task.clone());
				}
				self.show_tasks = !self.tasks_factory.guard().is_empty();
			},
			ContentInput::UpdateTask(index, task) => {
				service.update_task(task).expect("Failed to update task.");
				if let Some(index) = index {
					if self.parent_list.provider == "starred" {
						self.tasks_factory.guard().remove(index.current_index());
					}
				}
			},
		}
	}
}
