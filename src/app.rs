pub mod components;
pub mod factories;
pub mod models;
use std::str::FromStr;

use adw::glib::Propagation;
use core_done::service::Service;
use relm4::{
	actions::{ActionGroupName, RelmAction, RelmActionGroup},
	adw,
	adw::prelude::{AdwApplicationWindowExt, NavigationPageExt},
	component::{
		AsyncComponent, AsyncComponentController, AsyncComponentParts,
		AsyncController,
	},
	gtk::{
		self,
		prelude::{
			ApplicationExt, ApplicationExtManual, BoxExt, ButtonExt, Cast, FileExt,
		},
		traits::{ApplicationWindowExt, GtkWindowExt, OrientableExt, WidgetExt},
	},
	loading_widgets::LoadingWidgets,
	main_adw_application, new_action_group, new_stateless_action, view,
	AsyncComponentSender, ComponentBuilder, ComponentController, Controller,
	RelmWidgetExt,
};
use relm4_icons::icon_name;

use crate::{
	app::{
		components::{
			preferences::PreferencesComponentOutput,
			task_list_sidebar::TaskListSidebarOutput,
		},
		config::{info::PROFILE, setup},
	},
	fl,
};

use self::{
	components::{
		about_dialog::AboutDialog,
		content::{ContentInput, ContentModel},
		preferences::PreferencesComponentModel,
		task_list_sidebar::{TaskListSidebarInput, TaskListSidebarModel},
	},
	models::sidebar_list::SidebarList,
};

pub mod config;

new_action_group!(pub(super) WindowActionGroup, "win");
new_stateless_action!(pub(super) ShortcutsAction, WindowActionGroup, "show-help-overlay");
new_stateless_action!(AboutAction, WindowActionGroup, "about");
new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");
new_stateless_action!(QuitAction, WindowActionGroup, "quit");

pub struct Done {
	task_list_sidebar_controller: AsyncController<TaskListSidebarModel>,
	content_controller: AsyncController<ContentModel>,
	about_dialog: Controller<AboutDialog>,
	preferences: AsyncController<PreferencesComponentModel>,
	startup_failed: bool,
}

#[derive(Debug)]
pub enum AppInput {
	ServiceDisabled(Service),
	ListSelected(SidebarList, Service),
	ReloadSidebar(Service),
	CleanContent,
	Refresh,
	Quit,
}

#[relm4::component(pub async)]
impl AsyncComponent for Done {
	type CommandOutput = ();
	type Input = AppInput;
	type Output = ();
	type Init = ();

	view! {
		#[root]
		adw::ApplicationWindow {
			set_size_request: (450, 600),
			set_default_size: (800, 800),
			connect_close_request[sender] => move |_| {
				sender.input(AppInput::Quit);
				Propagation::Stop
			},

			#[wrap(Some)]
			set_help_overlay: shortcuts = &gtk::Builder::from_resource(
					"/dev/edfloreshz/Done/ui/gtk/help-overlay.ui"
			).object::<gtk::ShortcutsWindow>("help_overlay").unwrap() -> gtk::ShortcutsWindow {
				set_transient_for: Some(&root),
				set_application: Some(&main_adw_application()),
			},

			add_css_class?: if PROFILE == "Devel" {
				Some("devel")
			} else {
				None
			},

			add_breakpoint = adw::Breakpoint::new(adw::BreakpointCondition::new_length(
				adw::BreakpointConditionLengthType::MaxWidth,
				500.0,
				adw::LengthUnit::Sp,
			)) {
				add_setter: (
					&outter_view,
					"collapsed",
					&true.into(),
				),
			},

			if model.startup_failed {
				adw::ToolbarView {
					#[name = "content_header"]
					add_top_bar = &adw::HeaderBar {
						set_hexpand: true,
						set_css_classes: &["flat"],
						set_show_start_title_buttons: true,
						set_show_end_title_buttons: true,
						#[watch]
						set_title_widget: Some(&gtk::Label::new(
							Some("Error")
						)),
					},
					#[wrap(Some)]
					set_content = &gtk::Box {
						set_margin_all: 20,
						set_orientation: gtk::Orientation::Vertical,
						set_halign: gtk::Align::Center,
						set_valign: gtk::Align::Center,
						set_spacing: 10,
						gtk::Image {
							set_icon_name: Some(icon_name::WARNING),
							set_pixel_size: 64,
							set_margin_all: 10,
						},
						gtk::Label {
							set_css_classes: &["title-2"],
							set_wrap: true,
							set_wrap_mode: gtk::pango::WrapMode::Word,
							set_justify: gtk::Justification::Center,
							#[watch]
							set_text: fl!("error-ocurred"),
						},
						gtk::Label {
							set_css_classes: &["body"],
							#[watch]
							set_text: fl!("error-instructions"),
							set_wrap: true,
							set_wrap_mode: gtk::pango::WrapMode::Word,
							set_justify: gtk::Justification::Center,
						},
						gtk::Button {
							set_label: fl!("refresh-app"),
							set_valign: gtk::Align::End,
							connect_clicked => AppInput::Refresh
						},
						gtk::Label {
							set_css_classes: &["caption"],
							#[watch]
							set_text: fl!("restart-app"),
							set_wrap: true,
							set_wrap_mode: gtk::pango::WrapMode::Word,
							set_justify: gtk::Justification::Center,
						},
					}
				}
			} else {
				#[name(outter_view)]
				adw::NavigationSplitView {
					set_sidebar_width_fraction: 0.33,
					// #[wrap(Some)]
					// set_sidebar = &adw::NavigationPage {
					// 	#[name(inner_view)]
					// 	#[wrap(Some)]
					// 	set_child = &adw::NavigationSplitView {
					// 		set_max_sidebar_width: 260.0,
					// 		set_sidebar_width_fraction: 0.38,
					// 		#[wrap(Some)]
					// 		set_sidebar = &adw::NavigationPage {
					// 			set_title: "Services",
					// 			set_tag: Some("services-page"),
					// 			set_child: Some(model.services_sidebar_controller.widget()),
					// 		},
					// 		#[wrap(Some)]
					// 		set_content = &adw::NavigationPage {
					// 			set_title: "Lists",
					// 			set_tag: Some("lists-page"),
					// 			set_child: Some(model.task_list_sidebar_controller.widget()),
					// 		}
					// 	},
					// },
					#[wrap(Some)]
					set_sidebar = &adw::NavigationPage {
						set_title: "Lists",
						set_tag: Some("lists-page"),
						set_child: Some(model.task_list_sidebar_controller.widget()),
					},
					#[wrap(Some)]
					set_content = &adw::NavigationPage {
						set_title: "Tasks",
						set_tag: Some("content-page"),
						set_child: Some(model.content_controller.widget()),
					}
				}
			}
		}
	}

	fn init_loading_widgets(root: &mut Self::Root) -> Option<LoadingWidgets> {
		view! {
				#[local_ref]
				root {
					set_title: Some(fl!("done")),

					#[name(loading)]
					gtk::CenterBox {
						set_margin_all: 100,
						set_orientation: gtk::Orientation::Vertical,
						#[wrap(Some)]
						set_center_widget = &gtk::Picture {
							set_resource: Some("/dev/edfloreshz/Done/icons/scalable/apps/app-icon.svg"),
							set_margin_all: 150
						},
						#[wrap(Some)]
						set_end_widget = &gtk::Spinner {
							start: (),
							set_size_request: (40, 40),
							set_halign: gtk::Align::Center,
							set_valign: gtk::Align::Center,
						},
					}
				}
		}
		Some(LoadingWidgets::new(root, loading))
	}

	async fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: AsyncComponentSender<Self>,
	) -> AsyncComponentParts<Self> {
		let app = main_adw_application();
		let captured_sender = sender.clone();
		app.connect_open(move |_, files, _| {
			let bytes = files[0].uri();
			let uri = reqwest::Url::from_str(bytes.to_string().as_str()).unwrap();
			let captured_sender = captured_sender.clone();
			relm4::tokio::spawn(async move {
				let response = Service::Microsoft
					.get_service()
					.handle_uri_params(uri)
					.await;
				match response {
					Ok(_) => {
						captured_sender.input(AppInput::ReloadSidebar(Service::Microsoft));
						tracing::info!("Token stored");
					},
					Err(err) => tracing::error!("An error ocurred: {}", err),
				}
			});
		});

		let about_dialog = ComponentBuilder::default()
			.launch(root.upcast_ref::<gtk::Window>().clone())
			.detach();

		let mut model = Done {
			task_list_sidebar_controller: TaskListSidebarModel::builder()
				.launch(Service::Computer)
				.forward(sender.input_sender(), |message| match message {
					TaskListSidebarOutput::ServiceDisabled(service) => {
						AppInput::ServiceDisabled(service)
					},
					TaskListSidebarOutput::SelectList(list, service) => {
						AppInput::ListSelected(list, service)
					},
					TaskListSidebarOutput::CleanContent => AppInput::CleanContent,
				}),
			content_controller: ContentModel::builder().launch(None).detach(),
			about_dialog,
			preferences: PreferencesComponentModel::builder().launch(()).forward(
				sender.input_sender(),
				move |message| match message {
					PreferencesComponentOutput::ServiceDisabled(service) => {
						AppInput::ReloadSidebar(service)
					},
				},
			),
			startup_failed: false,
		};

		match setup::init_services() {
			Ok(_) => (),
			Err(_) => model.startup_failed = true,
		};

		let widgets = view_output!();

		let mut actions = RelmActionGroup::<WindowActionGroup>::new();

		let shortcuts_action = {
			let shortcuts = widgets.shortcuts.clone();
			RelmAction::<ShortcutsAction>::new_stateless(move |_| {
				shortcuts.present();
			})
		};

		let about_action = {
			let sender = model.about_dialog.sender().clone();
			RelmAction::<AboutAction>::new_stateless(move |_| {
				sender.send(()).unwrap_or_default();
			})
		};

		let preferences_action = {
			let window = model.preferences.widget().clone();
			RelmAction::<PreferencesAction>::new_stateless(move |_| {
				window.present();
			})
		};

		let quit_action = {
			let sender = sender.clone();
			RelmAction::<QuitAction>::new_stateless(move |_| {
				sender
					.input_sender()
					.send(Self::Input::Quit)
					.unwrap_or_default();
			})
		};

		actions.add_action(shortcuts_action);
		actions.add_action(about_action);
		actions.add_action(preferences_action);
		actions.add_action(quit_action);

		root.insert_action_group(
			WindowActionGroup::NAME,
			Some(&actions.into_action_group()),
		);

		AsyncComponentParts { model, widgets }
	}

	async fn update(
		&mut self,
		message: Self::Input,
		_sender: AsyncComponentSender<Self>,
		_root: &Self::Root,
	) {
		match message {
			AppInput::Quit => main_adw_application().quit(),
			AppInput::Refresh => {
				match setup::refresh() {
					Ok(_) => main_adw_application().quit(),
					Err(_) => main_adw_application().quit(),
				};
			},
			AppInput::ListSelected(list, service) => {
				self
					.content_controller
					.sender()
					.send(ContentInput::SelectList(list, service))
					.unwrap_or_default();
			},
			AppInput::CleanContent => self
				.content_controller
				.sender()
				.send(ContentInput::Clean)
				.unwrap_or_default(),
			AppInput::ServiceDisabled(service) => {
				self
					.content_controller
					.sender()
					.send(ContentInput::ServiceDisabled(service))
					.unwrap_or_default();
			},
			AppInput::ReloadSidebar(service) => self
				.task_list_sidebar_controller
				.sender()
				.send(TaskListSidebarInput::ReloadSidebar(service))
				.unwrap_or_default(),
		}
	}
}
