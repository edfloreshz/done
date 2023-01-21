use super::smart_lists::{SmartList, SmartListModel, SmartListOutput};
use crate::application::plugin::Plugin;
use crate::fl;
use crate::widgets::components::preferences::Preferences;
use crate::widgets::factory::list::ListFactoryModel;
use crate::widgets::factory::plugin::{
	PluginFactoryInit, PluginFactoryInput, PluginFactoryModel,
};
use libset::format::FileFormat;
use libset::project::Project;
use proto_rust::provider::List;
use relm4::adw::traits::PreferencesGroupExt;
use relm4::component::{
	AsyncComponentParts, AsyncComponentSender, SimpleAsyncComponent,
};
use relm4::factory::AsyncFactoryVecDeque;
use relm4::{adw, Component, ComponentController, Controller};
use relm4::{
	gtk,
	gtk::prelude::{BoxExt, OrientableExt, WidgetExt},
};

#[derive(Debug)]
pub struct SidebarComponentModel {
	provider_factory: AsyncFactoryVecDeque<PluginFactoryModel>,
	smart_list_controller: Controller<SmartListModel>,
	is_sidebar_empty: bool,
}

#[derive(Debug)]
pub enum SidebarComponentInput {
	AddListToProvider(usize, String, String),
	ListSelected(ListFactoryModel),
	EnableService(Plugin),
	DisableService(Plugin),
	RemoveService(Plugin),
	AddPluginToSidebar(Plugin),
	Forward,
	Notify(String),
	SelectSmartList(SmartList),
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum SidebarComponentOutput {
	ListSelected(ListFactoryModel),
	Forward,
	Notify(String, u32),
	DisablePlugin,
	SelectSmartList(SmartList),
}

#[relm4::component(pub async)]
impl SimpleAsyncComponent for SidebarComponentModel {
	type Input = SidebarComponentInput;
	type Output = SidebarComponentOutput;
	type Init = ();

	view! {
		sidebar = &gtk::Box {
			set_orientation: gtk::Orientation::Vertical,
			#[name(scroll_window)]
			gtk::ScrolledWindow {
				#[name(clamp)]
				adw::Clamp {
					#[wrap(Some)]
					set_child = &gtk::Box {
						set_orientation: gtk::Orientation::Vertical,
						set_css_classes: &["navigation-sidebar"],
						set_margin_top: 5,
						set_margin_start: 10,
						set_margin_end: 10,
						set_spacing: 12,
						set_vexpand: true,
						append = model.smart_list_controller.widget(),
						#[local_ref]
						providers_container -> adw::PreferencesGroup {
							set_hexpand: false,
							set_title: fl!("services"),
						},
						gtk::CenterBox {
							#[watch]
							set_visible: model.is_sidebar_empty,
							set_orientation: gtk::Orientation::Vertical,
							set_halign: gtk::Align::Center,
							set_vexpand: true,
							set_valign: gtk::Align::Start,
							set_margin_top: 15,
							#[wrap(Some)]
							set_center_widget = &gtk::Box {
								set_orientation: gtk::Orientation::Vertical,
								set_spacing: 24,
								gtk::Label {
									set_label: fl!("empty-sidebar"),
									set_css_classes: &["title-4", "accent"],
									set_wrap: true
								},
								gtk::Label {
									set_label: fl!("open-preferences"),
									set_wrap: true
								}
							}
						}
					}
				}
			},
		}
	}

	async fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: AsyncComponentSender<Self>,
	) -> AsyncComponentParts<Self> {
		let project = Project::open("dev", "edfloreshz", "done").unwrap();
		let preferences = project
			.get_file_as::<Preferences>("preferences", FileFormat::JSON)
			.unwrap();

		let mut model = SidebarComponentModel {
			provider_factory: AsyncFactoryVecDeque::new(
				adw::PreferencesGroup::default(),
				sender.input_sender(),
			),
			smart_list_controller: SmartListModel::builder().launch(()).forward(
				sender.input_sender(),
				|message| match message {
					SmartListOutput::SelectSmartList(list) => {
						SidebarComponentInput::SelectSmartList(list)
					},
					SmartListOutput::Forward => SidebarComponentInput::Forward,
				},
			),
			is_sidebar_empty: !preferences
				.plugins
				.iter()
				.any(|preferences| preferences.installed),
		};

		let providers_container = model.provider_factory.widget();

		let widgets = view_output!();

		for plugin_preference in
			preferences.plugins.iter().filter(|plugin| plugin.installed)
		{
			model
				.provider_factory
				.guard()
				.push_back(PluginFactoryInit::new(
					plugin_preference.plugin.clone(),
					plugin_preference.enabled,
				));
		}

		AsyncComponentParts { model, widgets }
	}

	async fn update(
		&mut self,
		message: Self::Input,
		sender: AsyncComponentSender<Self>,
	) {
		match message {
			SidebarComponentInput::AddListToProvider(index, provider_id, name) => {
				match Plugin::get_plugins()
					.unwrap()
					.iter()
					.find(|i| i.id == provider_id)
				{
					Some(provider) => {
						let provider = provider.clone();
						match relm4::spawn(async move { provider.connect().await })
							.await
							.unwrap()
						{
							Ok(mut service) => {
								let list = List::new(&name, &provider_id);
								let create_list = list.clone();
								match relm4::spawn(async move {
									service.create_list(create_list).await
								})
								.await
								.unwrap()
								{
									Ok(response) => {
										let response = response.into_inner();
										if response.successful {
											self
												.provider_factory
												.send(index, PluginFactoryInput::AddList(list));
										}
										sender
											.output(SidebarComponentOutput::Notify(
												response.message,
												1,
											))
											.unwrap_or_default();
									},
									Err(err) => {
										sender
											.output(SidebarComponentOutput::Notify(
												err.to_string(),
												2,
											))
											.unwrap_or_default();
									},
								}
							},
							Err(err) => {
								sender
									.output(SidebarComponentOutput::Notify(err.to_string(), 2))
									.unwrap_or_default();
							},
						}
					},
					None => {
						sender
							.output(SidebarComponentOutput::Notify(
								"Provider not found".to_string(),
								2,
							))
							.unwrap_or_default();
					},
				}
			},
			SidebarComponentInput::AddPluginToSidebar(plugin) => {
				match plugin.start() {
					Ok(_) => {
						self
							.provider_factory
							.guard()
							.push_back(PluginFactoryInit::new(plugin.clone(), true));
						self.is_sidebar_empty = false;
						tracing::info!("Added {:?} service to the sidebar", plugin.name);
					},
					Err(err) => sender
						.output_sender()
						.send(SidebarComponentOutput::Notify(err.to_string(), 2))
						.unwrap(),
				}
			},
			SidebarComponentInput::EnableService(plugin) => {
				let index = self
					.provider_factory
					.guard()
					.iter()
					.position(|p| p.map_or(false, |p| p.plugin == plugin));
				if let Some(index) = index {
					println!("ENABLED SERVICE CORRECTLY");
					self
						.provider_factory
						.send(index, PluginFactoryInput::Enable);
				}
			},
			SidebarComponentInput::DisableService(plugin) => {
				let index = self
					.provider_factory
					.guard()
					.iter()
					.position(|p| p.unwrap().plugin == plugin);
				if let Some(index) = index {
					self
						.provider_factory
						.send(index, PluginFactoryInput::Disable);
					sender
						.output(SidebarComponentOutput::DisablePlugin)
						.unwrap_or_default();
					let project = Project::open("dev", "edfloreshz", "done").unwrap();
					let preferences = project
						.get_file_as::<Preferences>("preferences", FileFormat::JSON)
						.unwrap();
					self.is_sidebar_empty = !preferences
						.plugins
						.iter()
						.any(|preferences| preferences.installed);
				}
			},
			SidebarComponentInput::RemoveService(plugin) => {
				let index = self
					.provider_factory
					.guard()
					.iter()
					.position(|p| p.unwrap().plugin == plugin);
				if let Some(index) = index {
					match self.provider_factory.guard().remove(index) {
						Some(provider) => {
							tracing::info!("Removed {} service", provider.plugin.name)
						},
						None => tracing::error!("Failed to remove service from sidebar."),
					}
				}
				if self.provider_factory.guard().is_empty() {
					self.is_sidebar_empty = true;
				}
			},
			SidebarComponentInput::ListSelected(list) => {
				sender
					.output(SidebarComponentOutput::ListSelected(list))
					.unwrap_or_default();
			},
			SidebarComponentInput::Forward => {
				sender
					.output(SidebarComponentOutput::Forward)
					.unwrap_or_default();
			},
			SidebarComponentInput::Notify(msg) => {
				sender
					.output(SidebarComponentOutput::Notify(msg, 2))
					.unwrap_or_default();
			},
			SidebarComponentInput::SelectSmartList(list) => sender
				.output(SidebarComponentOutput::SelectSmartList(list))
				.unwrap_or_default(),
		}
	}
}
