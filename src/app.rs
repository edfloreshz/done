use crate::application::info::{APP_ID, PROFILE};
use crate::application::plugin::Plugin;
use crate::application::setup::{self};
use crate::fl;
use crate::widgets::components::about_dialog::AboutDialog;
use crate::widgets::components::content::{
	ContentComponentInput, ContentComponentModel, ContentComponentOutput,
};
use crate::widgets::components::preferences::{
	PreferencesComponentModel, PreferencesComponentOutput,
};
use crate::widgets::components::sidebar::{
	SidebarComponentInput, SidebarComponentModel, SidebarComponentOutput,
};
use crate::widgets::components::smart_lists::SmartList;
use crate::widgets::components::welcome::WelcomeComponent;
use crate::widgets::factory::list::ListFactoryModel;
use gtk::prelude::*;
use once_cell::unsync::Lazy;
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
use sysinfo::{ProcessExt, System, SystemExt};

thread_local! {
	static APP: Lazy<adw::Application> = Lazy::new(|| { adw::Application::new(Some(APP_ID), gtk::gio::ApplicationFlags::empty())});
}

pub fn main_app() -> adw::Application {
	APP.with(|app| (*app).clone())
}

pub struct App {
	pub sidebar: AsyncController<SidebarComponentModel>,
	pub content: AsyncController<ContentComponentModel>,
	pub preferences: AsyncController<PreferencesComponentModel>,
	pub welcome: Controller<WelcomeComponent>,
	pub about_dialog: Option<Controller<AboutDialog>>,
	pub page_title: Option<String>,
	pub warning_revealed: bool,
}

#[derive(Debug)]
pub enum Event {
	TaskListSelected(ListFactoryModel),
	Notify(String, u32),
	EnablePluginOnSidebar(Plugin),
	AddPluginToSidebar(Plugin),
	DisablePluginOnSidebar(Plugin),
	RemovePluginFromSidebar(Plugin),
	SelectSmartList(SmartList),
	ToggleCompact(bool),
	DisablePlugin,
	CloseWarning,
	Folded,
	Unfolded,
	Forward,
	Back,
	Quit,
}

relm4::new_action_group!(pub(super) WindowActionGroup, "win");
relm4::new_stateless_action!(
	PreferencesAction,
	WindowActionGroup,
	"preferences"
);
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
				preferences => PreferencesAction,
				keyboard_shortcuts => ShortcutsAction,
				about_done => AboutAction,
				quit => QuitAction,
			}
		}
	}

	view! {
		#[root]
		main_window = adw::ApplicationWindow::new(&main_app()) {
			set_default_width: 700,
			set_default_height: 700,
			connect_close_request[sender] => move |_| {
				sender.input(Event::Quit);
				gtk::Inhibit(true)
			},

			#[wrap(Some)]
			set_help_overlay: shortcuts = &gtk::Builder::from_resource(
					"/dev/edfloreshz/Done/ui/gtk/help-overlay.ui"
			).object::<gtk::ShortcutsWindow>("help_overlay").unwrap() -> gtk::ShortcutsWindow {
				set_transient_for: Some(&main_window),
				set_application: Some(&crate::app::main_app()),
			},

			add_css_class?: if PROFILE == "Devel" {
				Some("devel")
			} else {
				None
			},

			gtk::Box {
				#[name = "overlay"]
				adw::ToastOverlay {
					#[wrap(Some)]
					set_child: stack = &gtk::Stack {
						set_hexpand: true,
						set_vexpand: true,
						set_transition_duration: 250,
						set_transition_type: gtk::StackTransitionType::Crossfade,
						add_child = &gtk::Box {
							set_orientation: gtk::Orientation::Vertical,
							append: leaflet = &adw::Leaflet {
								set_can_navigate_back: true,
								append: sidebar = &gtk::Box {
									set_orientation: gtk::Orientation::Vertical,
									set_width_request: 280,
									#[name = "sidebar_header"]
									adw::HeaderBar {
										set_show_end_title_buttons: false,
										set_title_widget: Some(&gtk::Label::new(Some("Done"))),
										pack_end = &gtk::MenuButton {
											set_icon_name: "open-menu-symbolic",
											set_menu_model: Some(&primary_menu),
										},
									},
									append: model.sidebar.widget(),
								},
								append: &gtk::Separator::default(),
								append: content = &gtk::Box {
									set_orientation: gtk::Orientation::Vertical,
									#[name = "content_header"]
									append = &adw::HeaderBar {
										set_hexpand: true,
										set_show_start_title_buttons: true,
										#[watch]
										set_title_widget: Some(&gtk::Label::new(model.page_title.as_deref())),
										pack_start: go_back_button = &gtk::Button {
											set_icon_name: "go-previous-symbolic",
											set_visible: false,
											connect_clicked[sender] => move |_| {
												sender.input(Event::Back);
											}
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
										gtk4::Label {
											set_wrap: true,
											set_natural_wrap_mode: gtk4::NaturalWrapMode::None,
											add_css_class: "warning",
											set_text: fl!("alpha-warning")
										}
									},
									append = &gtk::Box {
										#[watch]
										set_visible: model.page_title.is_none(),
										append: model.welcome.widget()
									},
									append = &gtk::Box {
										#[watch]
										set_visible: model.page_title.is_some(),
										append: model.content.widget()
									},
								},
								connect_folded_notify[sender] => move |leaflet| {
									if leaflet.is_folded() {
										sender.input(Event::Folded);
									} else {
										sender.input(Event::Unfolded);
									}
								}
							},
						}
					}
				}
			}
		}
	}

	fn init_loading_widgets(
		root: &mut Self::Root,
	) -> Option<relm4::loading_widgets::LoadingWidgets> {
		let icon = if PROFILE == "Devel" {
			"/dev/edfloreshz/Done/icons/scalable/apps/app-icon-devel.svg"
		} else {
			"/dev/edfloreshz/Done/icons/scalable/apps/app-icon.svg"
		};
		view! {
				#[local_ref]
				root {
					set_title: Some("Done"),
					set_default_size: (700, 700),

					#[name(loading)]
					gtk::CenterBox {
						set_margin_all: 100,
						set_orientation: gtk::Orientation::Vertical,
						#[wrap(Some)]
						set_center_widget = &gtk::Picture {
							set_resource: Some(icon),
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
		let mut model = App::new(sender.clone()).init_services().await;

		let preferences: &str = fl!("preferences");
		let keyboard_shortcuts: &str = fl!("keyboard-shortcuts");
		let about_done: &str = fl!("about-done");
		let quit: &str = fl!("quit");

		let widgets = view_output!();

		let preferences_action = {
			let preferences = model.preferences.widget().clone();
			RelmAction::<PreferencesAction>::new_stateless(move |_| {
				preferences.present();
			})
		};

		let shortcuts_action = {
			let shortcuts = widgets.shortcuts.clone();
			RelmAction::<ShortcutsAction>::new_stateless(move |_| {
				shortcuts.present();
			})
		};

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

		let actions = RelmActionGroup::<WindowActionGroup>::new();
		actions.add_action(&preferences_action);
		actions.add_action(&shortcuts_action);
		actions.add_action(&about_action);
		actions.add_action(&quit_action);

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
			Event::Quit => {
				//TODO: Terminate all running plugins.
				let processes = System::new_all();
				let mut local = processes.processes_by_exact_name("local-plugin");
				if let Some(process) = local.next() {
					if process.kill() {
						tracing::info!("The {} process was killed.", process.name());
					} else {
						tracing::error!("Failed to kill process.");
					}
				} else {
					tracing::info!("Process is not running.");
				}
				main_app().quit()
			},
			Event::CloseWarning => self.warning_revealed = false,
			Event::TaskListSelected(list) => {
				self.page_title = Some(list.list.clone().unwrap().name);
				self
					.content
					.sender()
					.send(ContentComponentInput::TaskListSelected(list))
					.unwrap_or_default();
			},
			Event::DisablePlugin => {
				self.page_title = None;
				self
					.content
					.sender()
					.send(ContentComponentInput::DisablePlugin)
					.unwrap_or_default();
			},
			Event::Notify(msg, timeout) => {
				widgets.overlay.add_toast(&toast(msg, timeout))
			},
			Event::Folded => {
				if self.page_title.is_some() {
					widgets.leaflet.set_visible_child(&widgets.content);
				} else {
					widgets.leaflet.set_visible_child(&widgets.sidebar);
				}
				widgets.go_back_button.set_visible(true);
				widgets.sidebar_header.set_show_start_title_buttons(true);
				widgets.sidebar_header.set_show_end_title_buttons(true);
			},
			Event::Unfolded => {
				widgets.go_back_button.set_visible(false);
				widgets.sidebar_header.set_show_start_title_buttons(false);
				widgets.sidebar_header.set_show_end_title_buttons(false);
			},
			Event::Forward => widgets.leaflet.set_visible_child(&widgets.content),
			Event::Back => widgets.leaflet.set_visible_child(&widgets.sidebar),
			Event::AddPluginToSidebar(plugin) => self
				.sidebar
				.sender()
				.send(SidebarComponentInput::AddPluginToSidebar(plugin))
				.unwrap(),
			Event::EnablePluginOnSidebar(plugin) => self
				.sidebar
				.sender()
				.send(SidebarComponentInput::EnableService(plugin))
				.unwrap_or_default(),
			Event::DisablePluginOnSidebar(plugin) => self
				.sidebar
				.sender()
				.send(SidebarComponentInput::DisableService(plugin))
				.unwrap_or_default(),
			Event::RemovePluginFromSidebar(plugin) => {
				self
					.sidebar
					.sender()
					.send(SidebarComponentInput::RemoveService(plugin))
					.unwrap_or_default();
				self
					.content
					.sender()
					.send(ContentComponentInput::DisablePlugin)
					.unwrap_or_default()
			},
			Event::SelectSmartList(list) => {
				self.page_title = Some(list.name());
				self
					.content
					.sender()
					.send(ContentComponentInput::SelectSmartList(list))
					.unwrap_or_default();
			},
			Event::ToggleCompact(compact) => self
				.content
				.sender()
				.send(ContentComponentInput::ToggleCompact(compact))
				.unwrap(),
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
