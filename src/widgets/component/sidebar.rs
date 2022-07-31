use std::collections::VecDeque;
use std::ops::Deref;

use diesel::SqliteConnection;
use relm4::{
	ComponentParts,
	ComponentSender,
	gtk, gtk::prelude::{BoxExt, ListBoxRowExt, OrientableExt, WidgetExt}, SimpleComponent, WidgetPlus,
};
use relm4::adw;
use relm4::factory::{DynamicIndex, FactoryVecDeque};

use crate::{fl, PLUGINS};
use crate::data::models::generic::lists::GenericList;
use crate::data::plugins::local::service::LocalService;
use crate::data::traits::provider::{ProviderService, TaskProvider};
// use crate::plugins::local::lists::{get_lists, post_list};
use crate::widgets::factory::list::ListType;

#[derive(Debug)]
pub struct SidebarModel {
	services: FactoryVecDeque<Box<dyn ProviderService>>,
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
	ListSelected(usize, String, GenericList),
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
				set_child: clamp = &adw::Clamp {
					#[wrap(Some)]
						set_child: providers_container = &gtk::Box {
							set_margin_top: 5,
							set_margin_start: 10,
							set_margin_end: 10,
							set_orientation: gtk::Orientation::Vertical,
							set_spacing: 12,
							set_vexpand: true,
							set_css_classes: &["navigation-sidebar"],
							// connect_row_activated[sender] => move |listbox, _| {
							// 	let index = listbox.selected_row().unwrap().index() as usize;
							// 	sender.input(SidebarInput::ListSelected(index));
							// 	sender.output(SidebarOutput::Forward)
							// },
						},
					}
			},
		}
	}

	fn init(
		_params: Self::InitParams,
		root: &Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let widgets = view_output!();
		let plugins = PLUGINS.get().unwrap();
		let mut model = SidebarModel {
			services: FactoryVecDeque::new(
				widgets.providers_container.clone(),
				&sender.input,
			),
		};
		if plugins.local.provider.get_enabled() {
			model.services.guard()
				.push_back(Box::new(plugins.local.clone()));
			// TODO: For each provider, retrieve the list of task lists.
		}
		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
		let mut guard = self.services.guard();
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
				let service = guard.get(index).unwrap();
				sender.output(SidebarOutput::ListSelected(
					index,
					service.get_provider().get_name().to_string(),
					service.get_task_lists().get(index).unwrap().clone(),
				));
			},
			SidebarInput::UpdateCounters(lists) => {
				for list in lists {
					match list {
						ListType::Inbox(i) => 0,
						ListType::Today(i) => 0,
						ListType::Next7Days(i) => 0,
						ListType::All(i) => 0,
						ListType::Starred(i) => 0,
						ListType::Archived(i) => 0,
						ListType::Other(index, i) => 0,
					};
				}
			},
		}
	}
}
