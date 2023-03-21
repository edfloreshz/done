use crate::application::plugin::Plugin;
use crate::factories::task_list::model::TaskListFactoryInit;
use crate::widgets::list_entry::{ListEntryModel, ListEntryOutput};
use crate::widgets::lists::helpers::add_list_to_provider;
use crate::widgets::lists::messages::{TaskListsInput, TaskListsOutput};
use crate::widgets::lists::model::TaskListsModel;
use relm4::component::{AsyncComponentParts, SimpleAsyncComponent};
use relm4::factory::AsyncFactoryVecDeque;
use relm4::gtk::traits::{ButtonExt, OrientableExt, WidgetExt};
use relm4::{adw, gtk, AsyncComponentSender, Component};

#[relm4::component(pub async)]
impl SimpleAsyncComponent for TaskListsModel {
	type Input = TaskListsInput;
	type Output = TaskListsOutput;
	type Init = Option<Plugin>;

	view! {
		#[root]
		gtk::Box {
			set_orientation: gtk::Orientation::Vertical,
			set_hexpand: true,
			adw::HeaderBar {
				set_css_classes: &["flat"],
				set_show_end_title_buttons: false,
				set_title_widget: Some(&gtk::Label::new(Some("Lists"))),
				pack_end = &gtk::Button {
					set_icon_name: "file-search-symbolic",
				},
				pack_start = &gtk::Button {
					set_icon_name: "value-increase-symbolic",
					set_css_classes: &["flat", "image-button"],
					set_valign: gtk::Align::Center,
					connect_clicked => TaskListsInput::AddTaskList(String::from("New list"))
				},
			},
			#[local_ref]
			list_widget -> gtk::ListBox {
				set_css_classes: &["navigation-sidebar"],
			}
		}
	}

	async fn init(
		init: Self::Init,
		root: Self::Root,
		sender: AsyncComponentSender<Self>,
	) -> AsyncComponentParts<Self> {
		let list_widget = gtk::ListBox::default();
		let list_factory =
			AsyncFactoryVecDeque::new(list_widget.clone(), sender.input_sender());

		let model = TaskListsModel {
			plugin: init,
			new_list_controller: ListEntryModel::builder().launch(()).forward(
				sender.input_sender(),
				move |message| match message {
					ListEntryOutput::AddTaskListToSidebar(name) => {
						TaskListsInput::AddTaskList(name)
					},
				},
			),
			list_factory,
			show_pane: true,
		};
		let widgets = view_output!();

		AsyncComponentParts { model, widgets }
	}

	async fn update(
		&mut self,
		message: Self::Input,
		sender: AsyncComponentSender<Self>,
	) {
		match message {
			TaskListsInput::PluginSelected(plugin) => {
				self.list_factory.guard().clear();
				self.plugin = Some(plugin.clone());
				match plugin.start().await {
					Ok(_) => {
						let mut client = plugin.connect().await.unwrap();
						let mut stream = client.get_lists(()).await.unwrap().into_inner();
						while let Some(response) = stream.message().await.unwrap() {
							self
								.list_factory
								.guard()
								.push_back(TaskListFactoryInit::new(
									plugin.clone(),
									response.list.unwrap(),
								));
						}
					},
					Err(err) => tracing::error!("Plugin couldn't be started. {err}"),
				}
			},
			TaskListsInput::AddTaskList(name) => {
				match add_list_to_provider(
					self,
					sender,
					self.plugin.as_ref().unwrap().clone(),
					name,
				)
				.await
				{
					Ok(_) => {
						tracing::info!(
							"List added to {}",
							self.plugin.as_ref().unwrap().name
						);
					},
					Err(err) => {
						tracing::error!("There was an error adding the list: {err}")
					},
				}
			},
			TaskListsInput::Forward => {
				sender.output(TaskListsOutput::Forward).unwrap()
			},
			TaskListsInput::ListSelected(list) => {
				sender.output(TaskListsOutput::ListSelected(list)).unwrap()
			},
			TaskListsInput::Notify(_msg) => {
				// sender.output(TaskListsOutput::Notify(msg)).unwrap()
			},
			TaskListsInput::DeleteTaskList(index, list_id) => {
				self.list_factory.guard().remove(index.current_index());
				tracing::info!("Deleted task list with id: {}", list_id);
			},
		}
	}
}
