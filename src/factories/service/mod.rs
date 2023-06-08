use done_local_storage::{models::list::List, service::Service};
use relm4::{
	factory::{AsyncFactoryComponent, AsyncFactoryVecDeque, FactoryView},
	gtk::{self, traits::WidgetExt},
	loading_widgets::LoadingWidgets,
	prelude::DynamicIndex,
	AsyncFactorySender, RelmWidgetExt,
};

use crate::widgets::sidebar::{
	messages::SidebarComponentInput, model::SidebarList,
};

use super::task_list::{
	messages::TaskListFactoryInput,
	model::{TaskListFactoryInit, TaskListFactoryModel},
};

#[derive(Debug)]
pub struct ServiceModel {
	pub list_factory: AsyncFactoryVecDeque<TaskListFactoryModel>,
	pub service: Service,
	pub extended: bool,
}

#[derive(Debug)]
pub enum ServiceInput {
	ToggleExtended(bool),
	AddTaskListToSidebar(String, Service),
	DeleteTaskList(DynamicIndex, String, Service),
	SelectList(SidebarList),
	Notify(String),
}

#[derive(Debug)]
pub enum ServiceOutput {
	SelectList(SidebarList),
	Notify(String),
}

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for ServiceModel {
	type ParentWidget = gtk::Box;
	type ParentInput = SidebarComponentInput;
	type Input = ServiceInput;
	type Output = ServiceOutput;
	type Init = (Service, bool);
	type CommandOutput = ();

	view! {
			#[root]
			gtk::Expander {
				set_margin_all: 5,
				add_css_class: "property",
				set_expanded: true,
				set_label: Some(&self.service.to_string()),
				#[wrap(Some)]
				set_child = &gtk::Box {
						#[local_ref]
						list_box -> gtk::ListBox {
								#[watch]
								set_width_request: if self.extended { 250 } else { 50 },
								set_css_classes: &["navigation-sidebar"],
								connect_row_selected => move |_, listbox_row| {
										if let Some(row) = listbox_row {
												row.activate();
										}
								},
						}
				}
			}
	}

	fn init_loading_widgets(root: &mut Self::Root) -> Option<LoadingWidgets> {
		relm4::view! {
			#[local_ref]
			root {
				set_expanded: true,
				#[name(spinner)]
				#[wrap(Some)]
				set_child = &gtk::Spinner {
					start: ()
				}
			}
		}
		Some(LoadingWidgets::new(root, spinner))
	}

	async fn init_model(
		init: Self::Init,
		_index: &DynamicIndex,
		sender: AsyncFactorySender<Self>,
	) -> Self {
		let (service, extended) = init;
		let mut list_factory =
			AsyncFactoryVecDeque::new(gtk::ListBox::new(), sender.input_sender());

		{
			let mut guard = list_factory.guard();

			if matches!(service, Service::Smart) {
				for smart_list in SidebarList::list() {
					tracing::warn!("Need to implement smart list {smart_list:?}");
					guard.push_back(TaskListFactoryInit::new(None, smart_list, true));
				}
			} else {
				match service.get_service().read_lists().await {
					Ok(lists) => {
						for list in lists {
							guard.push_back(TaskListFactoryInit::new(
								Some(service),
								SidebarList::Custom(list),
								false,
							));
						}
					},
					Err(err) => tracing::error!("{err}"),
				}
			}
		}

		Self {
			list_factory,
			service,
			extended,
		}
	}

	fn init_widgets(
		&mut self,
		_index: &DynamicIndex,
		root: &Self::Root,
		_returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
		_sender: AsyncFactorySender<Self>,
	) -> Self::Widgets {
		let list_box = self.list_factory.widget();

		let widgets = view_output!();
		widgets
	}

	async fn update(
		&mut self,
		message: Self::Input,
		sender: AsyncFactorySender<Self>,
	) {
		match message {
			ServiceInput::SelectList(sidebar_list) => {
				sender.output(ServiceOutput::SelectList(sidebar_list))
			},
			ServiceInput::ToggleExtended(extended) => {
				self.extended = extended;
				let guard = self.list_factory.guard();
				for index in 0..guard.len() {
					guard.send(index, TaskListFactoryInput::ToggleExtended(extended))
				}
			},
			ServiceInput::AddTaskListToSidebar(name, service) => {
				let mut provider = service.get_service();
				match provider
					.create_list(List::new(name.as_str(), service))
					.await
				{
					Ok(list) => {
						let mut guard = self.list_factory.guard();
						guard.push_back(TaskListFactoryInit::new(
							Some(service),
							SidebarList::Custom(list),
							false,
						));
					},
					Err(err) => sender.output(ServiceOutput::Notify(err.to_string())),
				}
			},
			ServiceInput::DeleteTaskList(index, id, service) => {
				let mut service = service.get_service();
				match service.delete_list(id).await {
					Ok(_) => {
						let mut guard = self.list_factory.guard();
						guard.remove(index.current_index());
					},
					Err(err) => {
						sender.output(ServiceOutput::Notify(err.to_string()));
					},
				}
			},
			ServiceInput::Notify(msg) => sender.output(ServiceOutput::Notify(msg)),
		}
	}

	fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
		match output {
			ServiceOutput::SelectList(list) => {
				Some(SidebarComponentInput::SelectList(list))
			},
			ServiceOutput::Notify(msg) => Some(SidebarComponentInput::Notify(msg)),
		}
	}
}
