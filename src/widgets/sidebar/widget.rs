use crate::factories::service::ServiceInput;
use crate::fl;
use crate::widgets::preferences::model::Preferences;
use done_local_storage::service::Service;
use libset::format::FileFormat;
use libset::project::Project;
use relm4::component::{
	AsyncComponentParts, AsyncComponentSender, SimpleAsyncComponent,
};
use relm4::factory::AsyncFactoryVecDeque;
use relm4::gtk::traits::{BoxExt, ButtonExt};
use relm4::loading_widgets::LoadingWidgets;
use relm4::RelmWidgetExt;
use relm4::{
	gtk,
	gtk::prelude::{OrientableExt, WidgetExt},
};

use super::messages::{SidebarComponentInput, SidebarComponentOutput};
use super::model::{SidebarComponentModel, SidebarList};

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
				services_list -> gtk::Box {
					set_orientation: gtk::Orientation::Vertical,
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

	fn init_loading_widgets(
		root: &mut Self::Root,
	) -> Option<relm4::loading_widgets::LoadingWidgets> {
		relm4::view! {
			#[local_ref]
			root {
				#[name(spinner)]
				gtk::Spinner {
					start: ()
				}
			}
		}
		Some(LoadingWidgets::new(root, spinner))
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
		let service_factory =
			AsyncFactoryVecDeque::new(gtk::Box::default(), sender.input_sender());
		let mut model = SidebarComponentModel {
			service_factory,
			extended: preferences.extended,
		};

		let services_list = model.service_factory.widget();

		let services = Service::list();

		let widgets = view_output!();

		{
			let mut guard = model.service_factory.guard();
			for service in services
				.iter()
				.filter(|service| service.get_service().available())
			{
				guard.push_back((*service, model.extended));
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
			SidebarComponentInput::ReloadSidebar => {
				let services = Service::list();
				let mut guard = self.service_factory.guard();
				guard.clear();

				for service in services
					.iter()
					.filter(|service| service.get_service().available())
				{
					guard.push_back((*service, self.extended));
				}
			},
			SidebarComponentInput::OpenPreferences => sender
				.output(SidebarComponentOutput::OpenPreferences)
				.unwrap_or_default(),
			SidebarComponentInput::SelectList(sidebar_list) => {
				if let SidebarList::Custom(list) = &sidebar_list {
					sender
						.output(SidebarComponentOutput::SelectList(
							sidebar_list.clone(),
							list.service,
						))
						.unwrap_or_default();
				} else {
					sender
						.output(SidebarComponentOutput::SelectList(
							sidebar_list,
							Service::Smart,
						))
						.unwrap_or_default();
				}
			},
			SidebarComponentInput::ToggleExtended(extended) => {
				self.extended = extended;
				let guard = self.service_factory.guard();
				for index in 0..guard.len() {
					guard.send(index, ServiceInput::ToggleExtended(extended))
				}
			},
			SidebarComponentInput::AddTaskListToSidebar(name, service) => {
				self
					.service_factory
					.send(0, ServiceInput::AddTaskListToSidebar(name, service));
			},
			SidebarComponentInput::Notify(msg) => sender
				.output(SidebarComponentOutput::Notify(msg, 1))
				.unwrap(),
		}
	}
}
