use core_done::service::Service;
use relm4::{
	adw,
	component::{
		AsyncComponent, AsyncComponentController, AsyncComponentParts,
		AsyncController,
	},
	factory::AsyncFactoryVecDeque,
	gtk::{
		self,
		prelude::{ButtonExt, OrientableExt},
		traits::{GtkWindowExt, WidgetExt},
	},
	AsyncComponentSender, RelmWidgetExt,
};
use relm4_icons::icon_name;

use crate::{
	app::{
		factories::service::ServiceFactoryModel, AboutAction, QuitAction,
		ShortcutsAction,
	},
	fl,
};

use super::preferences::{
	PreferencesComponentModel, PreferencesComponentOutput,
};

pub struct ServicesSidebarModel {
	services_factory: AsyncFactoryVecDeque<ServiceFactoryModel>,
	preferences: AsyncController<PreferencesComponentModel>,
}

#[derive(Debug)]
pub enum ServicesSidebarInput {
	ServiceSelected(Service),
	ReloadSidebar(Service),
	OpenPreferences,
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

	menu! {
		primary_menu: {
			section! {
				keyboard_shortcuts => ShortcutsAction,
				about_done => AboutAction,
				quit => QuitAction,
			}
		}
	}

	view! {
		#[root]
		adw::ToolbarView {
			#[name = "services_sidebar_header"]
			add_top_bar = &adw::HeaderBar {
				set_css_classes: &["flat"],
				set_show_start_title_buttons: true,
				pack_end = &gtk::MenuButton {
					set_tooltip: fl!("menu"),
					set_valign: gtk::Align::Center,
					set_css_classes: &["flat"],
					set_icon_name: icon_name::MENU,
					set_menu_model: Some(&primary_menu),
				},
				#[wrap(Some)]
				set_title_widget = &gtk::Label {
					set_hexpand: true,
					set_text: fl!("done"),
				},
			},
			#[wrap(Some)]
			set_content = &gtk::ScrolledWindow {
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
			add_bottom_bar = &adw::HeaderBar {
				set_show_start_title_buttons: false,
				set_show_end_title_buttons: false,
				#[wrap(Some)]
				set_title_widget = &gtk::Button {
					set_hexpand: true,
					set_css_classes: &["flat"],
					gtk::Box {
						set_halign: gtk::Align::Center,
						gtk::Image {
							set_icon_name: Some(icon_name::CONTROLS),
							set_margin_end: 10,
						},
						gtk::Label {
							set_text: fl!("preferences"),
						},
					},
					connect_clicked => ServicesSidebarInput::OpenPreferences
				},
			},
		},
	}

	async fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: AsyncComponentSender<Self>,
	) -> AsyncComponentParts<Self> {
		let keyboard_shortcuts: &str = fl!("keyboard-shortcuts");
		let about_done: &str = fl!("about-done");
		let quit: &str = fl!("quit");

		let mut services_factory =
			AsyncFactoryVecDeque::new(gtk::ListBox::default(), sender.input_sender());

		{
			let mut guard = services_factory.guard();

			for service in Service::list() {
				if service.get_service().available() {
					guard.push_back(service);
				}
			}
		}

		let model = ServicesSidebarModel {
			services_factory,
			preferences: PreferencesComponentModel::builder().launch(()).forward(
				sender.input_sender(),
				move |message| match message {
					PreferencesComponentOutput::ServiceDisabled(service) => {
						ServicesSidebarInput::ReloadSidebar(service)
					},
				},
			),
		};

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
			ServicesSidebarInput::OpenPreferences => {
				self.preferences.widget().present()
			},
		}
	}
}