use crate::factories::task_list::messages::TaskListFactoryInput;
use crate::factories::task_list::model::TaskListFactoryInit;
use crate::fl;
use crate::widgets::preferences::model::Preferences;
use crate::widgets::sidebar::model::SidebarList;
use done_local_storage::models::List;
use done_local_storage::LocalStorage;
use libset::format::FileFormat;
use libset::project::Project;
use relm4::component::{
	AsyncComponentParts, AsyncComponentSender, SimpleAsyncComponent,
};
use relm4::factory::AsyncFactoryVecDeque;
use relm4::gtk::traits::{BoxExt, ButtonExt};
use relm4::RelmWidgetExt;
use relm4::{
	gtk,
	gtk::prelude::{OrientableExt, WidgetExt},
};

use super::messages::{SidebarComponentInput, SidebarComponentOutput};
use super::model::SidebarComponentModel;

#[relm4::component(pub async)]
impl SimpleAsyncComponent for SidebarComponentModel {
	type Input = SidebarComponentInput;
	type Output = SidebarComponentOutput;
	type Init = ();

	view! {
		sidebar = &gtk::Box {
			set_hexpand: false,
			set_orientation: gtk::Orientation::Vertical,
			#[name(scroll_window)]
			gtk::ScrolledWindow {
				set_policy: (gtk::PolicyType::Never, gtk::PolicyType::Automatic),
				set_vexpand: true,
				#[local_ref]
				list_box -> gtk::ListBox {
					#[watch]
					set_width_request: if model.extended { 200 } else { 50 },
					set_css_classes: &["navigation-sidebar"],
					connect_row_selected => move |_, listbox_row| {
						if let Some(row) = listbox_row {
							row.activate();
						}
					},
				}
			},
			gtk::CenterBox {
				#[watch]
				set_visible: !model.extended,
				set_css_classes: &["navigation-sidebar"],
				set_tooltip: fl!("preferences"),
				#[wrap(Some)]
				set_center_widget = &gtk::Button {
					set_css_classes: &["flat"],
					set_width_request: 42,
					gtk::CenterBox {
						#[wrap(Some)]
						set_center_widget = &gtk::Image {
							set_icon_name: Some("controls")
						},
					},
					connect_clicked => SidebarComponentInput::OpenPreferences
				},
			},
			gtk::Box {
				#[watch]
				set_visible: model.extended,
				set_css_classes: &["navigation-sidebar"],
				set_tooltip: fl!("preferences"),
				set_margin_start: 5,
				set_margin_end: 5,
				gtk::Button {
					set_css_classes: &["flat"],
					gtk::Box {
						set_orientation: gtk::Orientation::Horizontal,
						gtk::Image {
							set_margin_all: 5,
							set_icon_name: Some("controls")
						},
						append = &gtk::Label {
							set_hexpand: true,
							set_text: fl!("preferences"),
							set_margin_all: 5,
						},
					},
					connect_clicked => SidebarComponentInput::OpenPreferences
				},
			}
		}
	}

	async fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: AsyncComponentSender<Self>,
	) -> AsyncComponentParts<Self> {
		let preferences =
			if let Ok(project) = Project::open("dev", "edfloreshz", "done") {
				project
					.get_file_as::<Preferences>("preferences", FileFormat::JSON)
					.unwrap_or(Preferences::new().await)
			} else {
				Preferences::new().await
			};
		let list_factory =
			AsyncFactoryVecDeque::new(gtk::ListBox::new(), sender.input_sender());
		let mut model = SidebarComponentModel {
			list_factory,
			extended: preferences.extended,
		};

		let local = LocalStorage::new();

		let list_box = model.list_factory.widget();
		let widgets = view_output!();

		{
			let mut guard = model.list_factory.guard();
			for smart_list in SidebarList::list() {
				guard.push_back(TaskListFactoryInit::new(smart_list, true));
			}

			if let Ok(lists) = local.get_lists().await {
				for list in lists {
					guard.push_back(TaskListFactoryInit::new(
						SidebarList::Custom(list),
						false,
					));
				}
			}
		}

		AsyncComponentParts { model, widgets }
	}

	async fn update(
		&mut self,
		message: Self::Input,
		sender: AsyncComponentSender<Self>,
	) {
		match message {
			SidebarComponentInput::OpenPreferences => sender
				.output(SidebarComponentOutput::OpenPreferences)
				.unwrap_or_default(),
			SidebarComponentInput::SelectList(list) => sender
				.output(SidebarComponentOutput::SelectList(list))
				.unwrap_or_default(),
			SidebarComponentInput::ToggleExtended(extended) => {
				self.extended = extended;
				let guard = self.list_factory.guard();
				for index in 0..guard.len() {
					guard.send(index, TaskListFactoryInput::ToggleExtended(extended))
				}
			},
			SidebarComponentInput::AddTaskListToSidebar(name) => {
				let local = LocalStorage::new();
				match local.create_list(List::new(name.as_str())).await {
					Ok(list) => {
						let mut guard = self.list_factory.guard();
						guard.push_back(TaskListFactoryInit::new(
							SidebarList::Custom(list),
							false,
						));
					},
					Err(err) => sender
						.output(SidebarComponentOutput::Notify(err.to_string(), 2))
						.unwrap_or_default(),
				}
			},
			SidebarComponentInput::DeleteTaskList(index, id) => {
				let local = LocalStorage::new();
				match local.delete_list(id).await {
					Ok(_) => {
						let mut guard = self.list_factory.guard();
						guard.remove(index.current_index());
					},
					Err(err) => {
						sender
							.output(SidebarComponentOutput::Notify(err.to_string(), 2))
							.unwrap_or_default();
					},
				}
			},
			SidebarComponentInput::Notify(msg) => sender
				.output(SidebarComponentOutput::Notify(msg, 1))
				.unwrap(),
		}
	}
}
