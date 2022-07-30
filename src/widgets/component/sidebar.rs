use std::collections::VecDeque;
use std::ops::Deref;
use relm4::factory::{DynamicIndex, FactoryVecDeque};
use relm4::{
	gtk,
	gtk::prelude::{BoxExt, ListBoxRowExt, OrientableExt, WidgetExt},
	ComponentParts, ComponentSender, SimpleComponent, WidgetPlus,
};
use crate::core::models::generic::lists::GenericList;
use crate::core::plugins::local::service::LocalService;
use crate::core::traits::provider::ProviderService;

// use crate::plugins::local::lists::{get_lists, post_list};
use crate::{fl, PLUGINS};
use crate::widgets::factory::list::ListType;

#[derive(Debug)]
pub struct SidebarModel {
	lists: FactoryVecDeque<GenericList>,
}

#[derive(Debug)]
pub enum SidebarInput {
	AddList(String, String),
	RemoveList(DynamicIndex),
	ListSelected(usize),
	UpdateCounters(Vec<ListType>),
}

#[derive(Debug)]
pub enum SidebarOutput {
	ListSelected(usize, GenericList),
	Forward,
}

#[relm4::component(pub)]
impl SimpleComponent for SidebarModel {
	type Input = SidebarInput;
	type Output = SidebarOutput;
	type InitParams = Option<GenericList>;
	type Widgets = SidebarWidgets;

	view! {
		sidebar = &gtk::Box {
			set_orientation: gtk::Orientation::Vertical,
			append: scroll_window = &gtk::ScrolledWindow {
				#[wrap(Some)]
				set_child: list = &gtk::ListBox {
					set_vexpand: true,
					set_margin_all: 2,
					set_css_classes: &["navigation-sidebar"],
					// connect_row_activated[sender] => move |listbox, _| {
					// 	let index = listbox.selected_row().unwrap().index() as usize;
					// 	sender.input(SidebarInput::ListSelected(index));
					// 	sender.output(SidebarOutput::Forward)
					// },
				},
			},
		}
	}

	fn init(
		_params: Self::InitParams,
		root: &Self::Root,
		sender: &ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let widgets = view_output!();
		let mut model = SidebarModel {
			lists: FactoryVecDeque::new(widgets.list.clone(), &sender.input),
		};
		let mut lists = vec![
			GenericList::new(fl!("inbox"), "document-save-symbolic", 0, "inbox"),
			GenericList::new(fl!("today"), "sun-alt-symbolic", 0, "today"),
			GenericList::new(fl!("next-7-days"), "org.gnome.Calendar.Devel-symbolic", 0, "next-7-days"),
			GenericList::new(
				fl!("all"),
				"edit-paste-symbolic",
				// get_all_tasks().unwrap_or_default().len() as i32,
				0,
				"all"
			),
			GenericList::new(
				fl!("starred"),
				"star-outline-rounded-symbolic",
				// get_favorite_tasks().unwrap_or_default().len() as i32,
				0,
				"starred"
			),
			GenericList::new(fl!("archived"), "folder-symbolic", 0, "archived"),
		];
		// TODO: For each provider, retrieve the list of task lists.
		lists.append(&mut PLUGINS.get().unwrap().local.read_task_lists().unwrap_or_default());
		for list in lists {
			model.lists.guard().push_back(list);
		}
		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, sender: &ComponentSender<Self>) {
		let mut guard = self.lists.guard();
		match message {
			SidebarInput::AddList(provider, name) => {
				// let posted_list = post_list(name).unwrap();
				// guard.push_back(posted_list)
			},
			SidebarInput::RemoveList(index) => {
				let index = index.current_index();
				guard.remove(index);
			},
			SidebarInput::ListSelected(index) => {
				let list = guard.get(index);
				sender.output(SidebarOutput::ListSelected(index, list.unwrap().clone()));
			},
			SidebarInput::UpdateCounters(lists) => {
				for list in lists {
					match list {
						ListType::Inbox(i) => guard.get_mut(0).unwrap().count += i as i32,
						ListType::Today(i) => guard.get_mut(1).unwrap().count += i as i32,
						ListType::Next7Days(i) => guard.get_mut(2).unwrap().count += i as i32,
						ListType::All(i) => guard.get_mut(3).unwrap().count += i as i32,
						ListType::Starred(i) => guard.get_mut(4).unwrap().count += i as i32,
						ListType::Archived(i) => guard.get_mut(5).unwrap().count += i as i32,
						ListType::Other(index, i) => {
							guard.get_mut(index).unwrap().count += i as i32
						},
					}
				}
			},
		}
	}
}
