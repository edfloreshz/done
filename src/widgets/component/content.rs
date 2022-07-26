use relm4::factory::{DynamicIndex, FactoryVecDeque};
use relm4::{
	gtk,
	gtk::prelude::{
		BoxExt, ButtonExt, EntryBufferExtManual, EntryExt, OrientableExt, WidgetExt,
	},
	view, ComponentParts, ComponentSender, SimpleComponent, WidgetPlus,
};

use crate::core::local::tasks::{
	delete_task, get_all_tasks, get_favorite_tasks, get_tasks, post_task,
};
use crate::models::list::List;
use crate::models::task::Task;
use crate::widgets::factory::list::ListType;
use crate::widgets::factory::list::ListType::{All, Other, Starred};
use crate::app::i18n::i18n;

pub struct ContentModel {
	parent_list: (usize, Option<List>),
	tasks: FactoryVecDeque<gtk::Box, Task, ContentInput>,
	show_tasks: bool,
}

pub enum ContentInput {
	AddTask(String),
	RemoveTask(DynamicIndex),
	RemoveWelcomeScreen,
	SetTaskList(usize, List),
	UpdateCounters(Vec<ListType>),
	FavoriteTask(DynamicIndex, bool),
}

pub enum ContentOutput {
	UpdateCounters(Vec<ListType>),
}

#[relm4::component(pub)]
impl SimpleComponent for ContentModel {
	type Input = ContentInput;
	type Output = ContentOutput;
	type InitParams = Option<Task>;
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
						set_text: &i18n("Tasks Will Appear Here")
					},
					append = &gtk::Button {
						#[watch]
						set_visible: model.parent_list.0 > 5,
						add_css_class: "suggested-action",
						set_label: &i18n("Add Tasks..."),
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
						set_placeholder_text: Some(&i18n("New task...")),
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
		sender: &ComponentSender<Self>,
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

	fn update(&mut self, message: Self::Input, sender: &ComponentSender<Self>) {
		match message {
			ContentInput::AddTask(title) => {
				let id_list = &self.parent_list.1.as_ref().unwrap().id_list;
				let task = post_task(id_list.clone().to_owned(), title.clone())
					.expect("Failed to post task.");
				self.tasks.push_back(task);

				sender.output(ContentOutput::UpdateCounters(vec![
					All(1),
					Other(self.parent_list.0, 1),
				]));
			},
			ContentInput::RemoveTask(index) => {
				if self.tasks.get(index.current_index()).favorite {
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
					let task = self.tasks.get(index.current_index());
					delete_task(&task.id_task).expect("Failed to remove task.");
				}
				self.tasks.remove(index.current_index());
			},
			ContentInput::RemoveWelcomeScreen => self.show_tasks = true,
			ContentInput::SetTaskList(index, list) => {
				self.parent_list = (index, Some(list.clone()));
				let tasks = match index {
					0 => vec![],
					1 => vec![],
					2 => vec![],
					3 => get_all_tasks().unwrap_or_default(),
					4 => get_favorite_tasks().unwrap_or_default(),
					_ => get_tasks(list.id_list.clone()).unwrap_or_default(),
				};
				loop {
					let task = self.tasks.pop_front();
					if task.is_none() {
						break;
					}
				}
				for task in tasks {
					self.tasks.push_back(task.clone());
				}
				self.show_tasks = !self.tasks.is_empty();
			},
			ContentInput::UpdateCounters(lists) => {
				sender.output(ContentOutput::UpdateCounters(lists))
			},
			ContentInput::FavoriteTask(index, favorite) => {
				if self.parent_list.0 == 4 {
					self.tasks.remove(index.current_index());
				}
				let value = if favorite { 1 } else { -1 };
				sender.output(ContentOutput::UpdateCounters(vec![Starred(value)]))
			},
		}
		self.tasks.render_changes();
	}
}
