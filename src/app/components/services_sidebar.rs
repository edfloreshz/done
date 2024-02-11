use core_done::service::Service;
use relm4::{
	adw,
	component::{AsyncComponent, AsyncComponentParts},
	factory::AsyncFactoryVecDeque,
	gtk::{self, prelude::OrientableExt, traits::WidgetExt},
	AsyncComponentSender,
};

use crate::{
	app::factories::service::{ServiceFactoryModel, ServiceFactoryOutput},
	fl,
};

pub struct ServicesSidebarModel {
	services_factory: AsyncFactoryVecDeque<ServiceFactoryModel>,
}

#[derive(Debug)]
pub enum ServicesSidebarInput {
	ServiceSelected(Service),
	ReloadSidebar(Service),
}

#[derive(Debug)]
pub enum ServicesSidebarOutput {
	ServiceSelected(Service),
	ServiceDisabled(Service),
}

#[relm4::component(pub async)]
impl AsyncComponent for ServicesSidebarModel {
	type CommandOutput = ();
	type Input = ServicesSidebarInput;
	type Output = ServicesSidebarOutput;
	type Init = ();

	view! {
		#[root]
		adw::ToolbarView {
			#[name = "services_sidebar_header"]
			add_top_bar = &adw::HeaderBar {
				set_css_classes: &["flat"],
				set_show_start_title_buttons: true,
				// pack_end = &gtk::MenuButton {
				// 	set_tooltip: fl!("menu"),
				// 	set_valign: gtk::Align::Center,
				// 	set_css_classes: &["flat"],
				// 	set_icon_name: icon_name::MENU,
				// 	set_menu_model: Some(&primary_menu),
				// },
				#[wrap(Some)]
				set_title_widget = &gtk::Label {
					set_hexpand: true,
					set_text: fl!("done"),
				},
			},
			#[wrap(Some)]
			set_content = &gtk::ScrolledWindow {
				set_direction: gtk::TextDirection::Ltr,
				gtk::Box {
					set_orientation: gtk::Orientation::Vertical,
					set_vexpand: true,
					#[local_ref]
					services_list -> gtk::ListBox {
						set_css_classes: &["navigation-sidebar"],
						connect_row_selected => move |_, listbox_row| {
							if let Some(row) = listbox_row {
								row.activate();
							}
						},
					},
				}
			},
		},
	}

	async fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: AsyncComponentSender<Self>,
	) -> AsyncComponentParts<Self> {
		let _keyboard_shortcuts: &str = fl!("keyboard-shortcuts");
		let _about_done: &str = fl!("about-done");
		let _quit: &str = fl!("quit");

		let mut services_factory = AsyncFactoryVecDeque::builder()
			.launch(gtk::ListBox::default())
			.forward(sender.input_sender(), |output| match output {
				ServiceFactoryOutput::ServiceSelected(service) => {
					ServicesSidebarInput::ServiceSelected(service)
				},
			});

		{
			let mut guard = services_factory.guard();

			for service in Service::list() {
				if service.get_service().available() {
					guard.push_back(service);
				}
			}
		}

		let model = ServicesSidebarModel { services_factory };

		let services_list = model.services_factory.widget();
		let widgets = view_output!();

		widgets
			.services_list
			.select_row(widgets.services_list.row_at_index(0).as_ref());

		AsyncComponentParts { model, widgets }
	}

	async fn update(
		&mut self,
		message: Self::Input,
		sender: AsyncComponentSender<Self>,
		_root: &Self::Root,
	) {
		match message {
			ServicesSidebarInput::ReloadSidebar(service) => {
				let mut guard = self.services_factory.guard();
				guard.clear();
				for service in Service::list() {
					if service.get_service().available() {
						guard.push_back(service);
					}
				}
				sender
					.output(ServicesSidebarOutput::ServiceDisabled(service))
					.unwrap()
			},
			ServicesSidebarInput::ServiceSelected(service) => {
				sender
					.output(ServicesSidebarOutput::ServiceSelected(service))
					.unwrap();
			},
		}
	}
}
