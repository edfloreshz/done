use crate::factories::plugin::model::PluginFactoryInit;
use crate::fl;
use crate::widgets::preferences::model::Preferences;
use crate::widgets::smart_lists::sidebar::model::SmartList;
use libset::format::FileFormat;
use libset::project::Project;
use relm4::component::{
	AsyncComponentParts, AsyncComponentSender, SimpleAsyncComponent,
};
use relm4::factory::AsyncFactoryVecDeque;
use relm4::gtk::traits::ButtonExt;
use relm4::{
	gtk,
	gtk::prelude::{ListBoxRowExt, OrientableExt, WidgetExt},
};
use relm4_icons::icon_name;

use super::helpers::{
	add_plugin_to_sidebar, disable_service, enable_service, remove_service,
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
			set_orientation: gtk::Orientation::Vertical,
			#[name(scroll_window)]
			gtk::ScrolledWindow {
				#[wrap(Some)]
				set_child = &gtk::Box {
					set_orientation: gtk::Orientation::Vertical,
					set_vexpand: true,
					#[local_ref]
					providers_container -> gtk::ListBox {
						set_css_classes: &["navigation-sidebar"],
						connect_row_selected => move |_, listbox_row| {
							if let Some(row) = listbox_row {
								row.activate();
							}
						},
						gtk::ListBoxRow {
							set_has_tooltip: true,
							set_tooltip_text: Some(fl!("all")),
							gtk::CenterBox {
								set_css_classes: &["plugin"],
								#[wrap(Some)]
								set_center_widget = &gtk::Image {
									set_icon_name: Some(icon_name::CLIPBOARD)
								},
							},
							connect_activate => SidebarComponentInput::SelectSmartList(SmartList::All)
						},
						gtk::ListBoxRow {
							set_has_tooltip: true,
							set_tooltip_text: Some(fl!("today")),
							gtk::CenterBox {
								set_css_classes: &["plugin"],
								#[wrap(Some)]
								set_center_widget = &gtk::Image {
									set_icon_name: Some(icon_name::IMAGE_ADJUST_BRIGHTNESS)
								},
							},
							connect_activate => SidebarComponentInput::SelectSmartList(SmartList::Today)
						},
						gtk::ListBoxRow {
							set_has_tooltip: true,
							set_tooltip_text: Some(fl!("starred")),
							gtk::CenterBox {
								set_css_classes: &["plugin"],
								#[wrap(Some)]
								set_center_widget = &gtk::Image {
									set_icon_name: Some(icon_name::STAR_FILLED_ROUNDED)
								},
							},
							connect_activate => SidebarComponentInput::SelectSmartList(SmartList::Starred)
						},
						gtk::ListBoxRow {
							set_has_tooltip: true,
							set_tooltip_text: Some(fl!("next-7-days")),
							gtk::CenterBox {
								set_css_classes: &["plugin"],
								#[wrap(Some)]
								set_center_widget = &gtk::Image {
									set_icon_name: Some(icon_name::WORK_WEEK)
								},
							},
							connect_activate => SidebarComponentInput::SelectSmartList(SmartList::Next7Days)
						},
					},
					gtk::CenterBox {
						set_vexpand: true,
						set_valign: gtk::Align::End,
						set_css_classes: &["navigation-sidebar"],
						set_has_tooltip: true,
						set_tooltip_text: Some("Preferences"),
						#[wrap(Some)]
						set_center_widget = &gtk::Button {
							set_css_classes: &["flat"],
							gtk::CenterBox {
								#[wrap(Some)]
								set_center_widget = &gtk::Image {
									set_icon_name: Some("controls")
								},
							},
							connect_clicked => SidebarComponentInput::OpenPreferences
						},
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
			plugin_factory: AsyncFactoryVecDeque::new(
				gtk::ListBox::default(),
				sender.input_sender(),
			),
		};

		let providers_container = model.plugin_factory.widget();

		let widgets = view_output!();

		for plugin_preference in
			preferences.plugins.iter().filter(|plugin| plugin.installed)
		{
			let plugin_name = plugin_preference.plugin.name.clone();
			if plugin_preference.enabled {
				match plugin_preference.plugin.start().await {
					Ok(_) => {
						tracing::info!("{plugin_name} plugin started.");
					},
					Err(_) => {
						tracing::error!("{plugin_name} plugin was not able to start.");
						sender
							.output(SidebarComponentOutput::Notify(
								"We had trouble starting some services, try restarting the app"
									.into(),
								2,
							))
							.unwrap();
					},
				}
			}
			model
				.plugin_factory
				.guard()
				.push_back(PluginFactoryInit::new(
					plugin_preference.plugin.clone(),
					plugin_preference.enabled,
				));

			if plugin_preference.enabled {
				match plugin_preference.plugin.connect().await {
					Ok(_) => continue,
					Err(_) => {
						sender.output(SidebarComponentOutput::Notify(format!("{plugin_name} service had trouble starting up, try updating the service or restarting the app."), 2)).unwrap();
					},
				}
			}
		}

		let row: Option<gtk::ListBoxRow> =
			widgets.providers_container.row_at_index(0);
		widgets.providers_container.select_row(row.as_ref());

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
			SidebarComponentInput::PluginSelected(plugin) => sender
				.output(SidebarComponentOutput::PluginSelected(plugin))
				.unwrap(),
			SidebarComponentInput::AddPluginToSidebar(plugin) => {
				if let Err(err) = add_plugin_to_sidebar(self, plugin).await {
					tracing::error!("{err}");
				}
			},
			SidebarComponentInput::EnableService(plugin) => {
				enable_service(self, plugin)
			},
			SidebarComponentInput::DisableService(plugin) => {
				if let Err(err) = disable_service(self, sender, plugin) {
					tracing::error!("{err}");
				}
			},
			SidebarComponentInput::RemoveService(plugin) => {
				if let Err(err) = remove_service(self, plugin.clone()) {
					tracing::error!("{err}");
				}
				if self.plugin_factory.guard().is_empty() {
					sender
						.output(SidebarComponentOutput::RemoveService(plugin))
						.unwrap_or_default()
				}
			},
			SidebarComponentInput::SelectSmartList(list) => sender
				.output(SidebarComponentOutput::SelectSmartList(list))
				.unwrap_or_default(),
		}
	}
}
