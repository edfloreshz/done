use relm4::{
	ComponentParts,
	ComponentSender,
	gtk, gtk::prelude::{
		BoxExt, ButtonExt, EntryBufferExtManual, EntryExt, OrientableExt, WidgetExt,
	}, SimpleComponent, view, WidgetPlus,
};
use relm4::factory::{DynamicIndex, FactoryVecDeque};

use crate::data::models::generic::lists::GenericList;
use crate::data::models::generic::tasks::GenericTask;
use crate::fl;
use crate::widgets::factory::list::ListType;
use crate::widgets::factory::list::ListType::{All, Other, Starred};

pub struct ContentModel {
	parent_list: (usize, Option<GenericList>),
	tasks: FactoryVecDeque<GenericTask>,
	show_tasks: bool,
}

#[derive(Debug)]
pub enum ContentInput {
	AddTask(String),
	RemoveTask(DynamicIndex),
	RemoveWelcomeScreen,
	SetTaskList(usize, String, GenericList),
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
						set_visible: model.parent_list.0 > 5,
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
						set_visible: model.parent_list.0 > 5,
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
			parent_list: (0, None),
			tasks: FactoryVecDeque::new(list_box.clone(), &sender.input),
			show_tasks: false,
		};
		let widgets = view_output!();
		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
		let mut guard = self.tasks.guard();
		match message {
			ContentInput::AddTask(title) => {
				let id_list = &self.parent_list.1.as_ref().unwrap().id_list;
				let task =
					// post_task(id_list.to_owned(), title).expect("Failed to post task.");
					// self.tasks.push_back(task);

					sender.output(ContentOutput::UpdateCounters(vec![
						All(1),
						Other(self.parent_list.0, 1),
					]));
			},
			ContentInput::RemoveTask(index) => {
				if guard.get(index.current_index()).unwrap().favorite {
					sender.output(ContentOutput::UpdateCounters(vec![
						All(-1),
						Starred(-1),
						Other(self.parent_list.0, -1),
					]));
				} else {
					sender.output(ContentOutput::UpdateCounters(vec![
						All(-1),
						Other(self.parent_list.0, -1),
					]));
				}
				{
					let task = guard.get(index.current_index());
					// delete_task(&task.id_task).expect("Failed to remove task.");
				}
				guard.remove(index.current_index());
			},
			ContentInput::RemoveWelcomeScreen => self.show_tasks = true,
			ContentInput::SetTaskList(index, provider, list) => {
				self.parent_list = (index, Some(list.clone()));
				let tasks = match index {
					0 => todo!("Get tasks from `Inbox` provider."),
					1 => todo!("Get tasks from `Today` provider."),
					2 => todo!("Get tasks from `Next7Days` provider."),
					3 => todo!("Get tasks from `All` provider."),
					4 => todo!("Get tasks from `Favorites` provider."),
					_ => todo!("Get specific task list."),
				};
				loop {
					let task = guard.pop_front();
					if task.is_none() {
						break;
					}
				}
				// for task in tasks {
				// 	guard.push_back(task.clone());
				// }
				self.show_tasks = !guard.is_empty();
			},
			ContentInput::UpdateCounters(lists) => {
				sender.output(ContentOutput::UpdateCounters(lists))
			},
			ContentInput::FavoriteTask(index, favorite) => {
				if self.parent_list.0 == 4 {
					guard.remove(index.current_index());
				}
				let value = if favorite { 1 } else { -1 };
				sender.output(ContentOutput::UpdateCounters(vec![Starred(value)]))
			},
		}
	}
}
