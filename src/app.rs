use crate::application::info::PROFILE;
use crate::application::setup::{self};
use crate::fl;
use crate::widgets::about_dialog::AboutDialog;
use crate::widgets::content::messages::{ContentInput, ContentOutput};
use crate::widgets::content::model::ContentModel;
use crate::widgets::preferences::messages::PreferencesComponentOutput;
use crate::widgets::preferences::model::{
	Preferences, PreferencesComponentModel,
};
use crate::widgets::sidebar::messages::{
	SidebarComponentInput, SidebarComponentOutput,
};
use crate::widgets::sidebar::model::SidebarComponentModel;
use crate::widgets::sidebar::model::SidebarList;

use crate::widgets::list_dialog::messages::ListDialogOutput;
use crate::widgets::list_dialog::model::ListDialogComponent;
use crate::widgets::welcome::WelcomeComponent;
use done_local_storage::service::Service;
use gtk::prelude::*;
use libset::format::FileFormat;
use libset::project::Project;
use relm4::adw::Toast;
use relm4::component::{AsyncComponentParts, AsyncController};
use relm4::loading_widgets::LoadingWidgets;
use relm4::{
	actions::{ActionGroupName, RelmAction, RelmActionGroup},
	adw,
	component::{AsyncComponent, AsyncComponentController},
	gtk, Component, ComponentBuilder, ComponentController, Controller,
};
use relm4::{view, AsyncComponentSender, RelmWidgetExt};
use relm4_icons::icon_name;

pub struct App {
	sidebar: AsyncController<SidebarComponentModel>,
	content: AsyncController<ContentModel>,
	preferences: AsyncController<PreferencesComponentModel>,
	welcome: Controller<WelcomeComponent>,
	list_entry: Controller<ListDialogComponent>,
	about_dialog: Option<Controller<AboutDialog>>,
	page_title: Option<String>,
	warning_revealed: bool,
	extended: bool,
}

impl App {
	pub fn new(
		sidebar: AsyncController<SidebarComponentModel>,
		content: AsyncController<ContentModel>,
		preferences: AsyncController<PreferencesComponentModel>,
		welcome: Controller<WelcomeComponent>,
		list_entry: Controller<ListDialogComponent>,
		about_dialog: Option<Controller<AboutDialog>>,
		extended: bool,
	) -> Self {
		let app = Self {
			sidebar,
			content,
			preferences,
			welcome,
			list_entry,
			about_dialog,
			page_title: None,
			warning_revealed: true,
			extended,
		};
		app
	}
}

#[derive(Debug)]
pub enum Event {
	Notify(String, u32),
	SelectList(SidebarList, Option<Service>),
	AddTaskList,
	AddTaskListToSidebar(String, Service),
	ToggleExtended(bool),
	OpenPreferences,
	DisablePlugin,
	CloseWarning,
	Quit,
}

relm4::new_action_group!(pub(super) WindowActionGroup, "win");
relm4::new_stateless_action!(pub(super) ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");
relm4::new_stateless_action!(QuitAction, WindowActionGroup, "quit");

#[relm4::component(pub async)]
impl AsyncComponent for App {
	type CommandOutput = ();
	type Input = Event;
	type Output = ();
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
		main_window = adw::ApplicationWindow {
			set_default_size: (700, 700),
			connect_close_request[sender] => move |_| {
				sender.input(Event::Quit);
				gtk::Inhibit(true)
			},

			// #[wrap(Some)]
			// set_help_overlay: shortcuts = &gtk::Builder::from_resource(
			// 		"/dev/edfloreshz/Done/ui/gtk/help-overlay.ui"
			// ).object::<gtk::ShortcutsWindow>("help_overlay").unwrap() -> gtk::ShortcutsWindow {
			// 	set_transient_for: Some(&main_window),
			// 	set_application: Some(&crate::setup::main_app()),
			// },

			add_css_class?: if PROFILE == "Devel" {
				Some("devel")
			} else {
				None
			},

			gtk::Box {
				set_orientation: gtk::Orientation::Horizontal,
				#[name = "sidebar"]
				gtk::Box {
					set_css_classes: &["view"],
					set_orientation: gtk::Orientation::Vertical,
					#[watch]
					set_width_request: if model.extended { 200 } else { 50 },
					#[name = "services_sidebar_header"]
					adw::HeaderBar {
						#[watch]
						set_visible: model.extended,
						set_css_classes: &["flat"],
						set_show_end_title_buttons: false,
						set_show_start_title_buttons: false,
						pack_start = &gtk::MenuButton {
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
						pack_end = &gtk::Button {
							set_tooltip: fl!("add-new-task-list"),
							set_icon_name: icon_name::PLUS,
							set_css_classes: &["flat", "image-button"],
							set_valign: gtk::Align::Center,
							connect_clicked => Event::AddTaskList
						},
					},
					gtk::CenterBox {
						#[watch]
						set_visible: !model.extended,
						set_height_request: 46,
						set_margin_top: 8,
						set_margin_bottom: 8,
						#[wrap(Some)]
						set_center_widget = &gtk::Box {
							set_spacing: 5,
							set_orientation: gtk::Orientation::Vertical,
							gtk::MenuButton {
								set_width_request: 42,
								set_valign: gtk::Align::Center,
								set_css_classes: &["flat"],
								set_icon_name: icon_name::MENU,
								set_menu_model: Some(&primary_menu),
							},
							gtk::Button {
								set_width_request: 42,
								set_tooltip: fl!("add-new-task-list"),
								set_icon_name: icon_name::PLUS,
								set_css_classes: &["flat", "image-button"],
								set_valign: gtk::Align::Center,
								connect_clicked => Event::AddTaskList
							}
						},
					},
					gtk::Separator::default(),
					append: model.sidebar.widget(),
				},
				gtk::Separator::default(),
				#[name(content)]
				gtk::Box {
					set_orientation: gtk::Orientation::Vertical,
					#[name = "content_header"]
					append = &adw::HeaderBar {
						set_hexpand: true,
						set_css_classes: &["flat"],
						set_show_start_title_buttons: false,
						set_show_end_title_buttons: true,
						#[watch]
						set_title_widget: Some(&gtk::Label::new(model.page_title.as_deref())),
						pack_start: go_back_button = &gtk::Button {
							set_tooltip: fl!("back"),
							set_icon_name: icon_name::LEFT,
							set_visible: false,
						},
						pack_start = &gtk::Button {
							set_visible: false,
							set_tooltip: fl!("search"),
							set_icon_name: icon_name::LOUPE,
						},
					},
					#[name(overlay)]
					adw::ToastOverlay {
						#[wrap(Some)]
						set_child = &gtk::Box {
							gtk::Box {
								#[watch]
								set_visible: model.page_title.is_none(),
								append: model.welcome.widget()
							},
							gtk::Box {
								#[watch]
								set_visible: model.page_title.is_some(),
								append: model.content.widget()
							},
						}
					},
					gtk::InfoBar {
						set_message_type: gtk::MessageType::Warning,
						set_visible: PROFILE == "Devel",
						#[watch]
						set_revealed: model.warning_revealed,
						set_show_close_button: true,
						connect_response[sender] => move |_, _| {
							sender.input_sender().send(Event::CloseWarning).unwrap()
						},
						gtk::Label {
							set_wrap: true,
							set_natural_wrap_mode: gtk::NaturalWrapMode::None,
							add_css_class: "warning",
							set_text: fl!("alpha-warning")
						}
					},
				},
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
		match setup::init_services().await {
			Ok(_) => (),
			Err(_) => panic!("Failed to initialize services."),
		};

		let keyboard_shortcuts: &str = fl!("keyboard-shortcuts");
		let about_done: &str = fl!("about-done");
		let quit: &str = fl!("quit");

		let mut actions = RelmActionGroup::<WindowActionGroup>::new();

		let preferences_controller = PreferencesComponentModel::builder()
			.launch(())
			.forward(sender.input_sender(), |message| match message {
				PreferencesComponentOutput::ToggleExtended(extended) => {
					Event::ToggleExtended(extended)
				},
			});

		let sidebar_controller = SidebarComponentModel::builder()
			.launch(())
			.forward(sender.input_sender(), |message| match message {
				SidebarComponentOutput::OpenPreferences => Event::OpenPreferences,
				SidebarComponentOutput::DisablePlugin => Event::DisablePlugin,
				SidebarComponentOutput::Notify(msg, timeout) => {
					Event::Notify(msg, timeout)
				},
				SidebarComponentOutput::SelectList(list, service) => {
					Event::SelectList(list, service)
				},
			});

		let content_controller = ContentModel::builder().launch(None).forward(
			sender.input_sender(),
			|message| match message {
				ContentOutput::Notify(msg, timeout) => Event::Notify(msg, timeout),
			},
		);

		let welcome_controller = WelcomeComponent::builder().launch(()).detach();
		let list_entry_controller = ListDialogComponent::builder()
			.launch(None)
			.forward(sender.input_sender(), |message| match message {
				ListDialogOutput::AddTaskListToSidebar(name, service) => {
					Event::AddTaskListToSidebar(name, service)
				},
				ListDialogOutput::RenameList(_name, _service) => todo!(),
			});

		let current_preferences =
			if let Ok(project) = Project::open("dev", "edfloreshz", "done") {
				project
					.get_file_as::<Preferences>("preferences", FileFormat::JSON)
					.unwrap_or(Preferences::new().await)
			} else {
				Preferences::new().await
			};

		let mut model = App::new(
			sidebar_controller,
			content_controller,
			preferences_controller,
			welcome_controller,
			list_entry_controller,
			None,
			current_preferences.extended,
		);

		let widgets = view_output!();

		// let shortcuts_action = {
		// 	let shortcuts = widgets.shortcuts.clone();
		// 	RelmAction::<ShortcutsAction>::new_stateless(move |_| {
		// 		shortcuts.present();
		// 	})
		// };

		let about_dialog = ComponentBuilder::default()
			.launch(widgets.main_window.upcast_ref::<gtk::Window>().clone())
			.detach();

		model.about_dialog = Some(about_dialog);

		let about_action = {
			let sender = model.about_dialog.as_ref().unwrap().sender().clone();
			RelmAction::<AboutAction>::new_stateless(move |_| {
				sender.send(()).unwrap_or_default();
			})
		};

		let quit_action = {
			RelmAction::<QuitAction>::new_stateless(move |_| {
				sender.input_sender().send(Event::Quit).unwrap_or_default();
			})
		};

		// actions.add_action(shortcuts_action);
		actions.add_action(about_action);
		actions.add_action(quit_action);

		widgets.main_window.insert_action_group(
			WindowActionGroup::NAME,
			Some(&actions.into_action_group()),
		);

		AsyncComponentParts { model, widgets }
	}

	async fn update_with_view(
		&mut self,
		widgets: &mut Self::Widgets,
		message: Self::Input,
		sender: AsyncComponentSender<Self>,
		_root: &Self::Root,
	) {
		match message {
			Event::Quit => (),
			Event::AddTaskList => {
				let list_entry = self.list_entry.widget();
				list_entry.present();
			},
			Event::AddTaskListToSidebar(name, service) => self
				.sidebar
				.sender()
				.send(SidebarComponentInput::AddTaskListToSidebar(name, service))
				.unwrap_or_default(),
			Event::OpenPreferences => {
				let preferences = self.preferences.widget();
				preferences.present();
			},
			Event::CloseWarning => self.warning_revealed = false,
			Event::DisablePlugin => {
				self.page_title = None;
				self
					.content
					.sender()
					.send(ContentInput::DisablePlugin)
					.unwrap_or_default();
			},
			Event::Notify(msg, timeout) => {
				widgets.overlay.add_toast(toast(msg, timeout))
			},
			Event::SelectList(list, service) => {
				self.page_title = Some(list.name());
				self
					.content
					.sender()
					.send(ContentInput::SelectList(list, service))
					.unwrap_or_default();
			},
			Event::ToggleExtended(extended) => {
				self.extended = extended;
				self
					.sidebar
					.sender()
					.send(SidebarComponentInput::ToggleExtended(extended))
					.unwrap()
			},
		}
		self.update_view(widgets, sender);
	}
}

pub fn toast<T: ToString>(title: T, timeout: u32) -> Toast {
	Toast::builder()
		.title(title.to_string().as_str())
		.timeout(timeout)
		.build()
}
