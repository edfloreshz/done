use crate::application::info::PROFILE;
use crate::application::plugin::Plugin;
use crate::application::setup::main_app;
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
use relm4::adw::Toast;
use relm4::component::AsyncController;
use relm4::{
	actions::{ActionGroupName, RelmAction, RelmActionGroup},
	adw,
	component::{AsyncComponent, AsyncComponentController},
	gtk, Component, ComponentBuilder, ComponentController, ComponentParts,
	ComponentSender, Controller,
};
use sysinfo::{ProcessExt, System, SystemExt};

pub struct App {
	sidebar: AsyncController<SidebarComponentModel>,
	content: AsyncController<ContentComponentModel>,
	preferences: AsyncController<PreferencesComponentModel>,
	welcome: Controller<WelcomeComponent>,
	about_dialog: Option<Controller<AboutDialog>>,
	page_title: Option<String>,
	warning_revealed: bool,
}

impl App {
	pub fn new(
		sidebar: AsyncController<SidebarComponentModel>,
		content: AsyncController<ContentComponentModel>,
		preferences: AsyncController<PreferencesComponentModel>,
		welcome: Controller<WelcomeComponent>,
		about_dialog: Option<Controller<AboutDialog>>,
	) -> Self {
		Self {
			sidebar,
			content,
			preferences,
			welcome,
			about_dialog,
			page_title: None,
			warning_revealed: true,
		}
	}
}

#[derive(Debug)]
pub enum Event {
	TaskListSelected(ListFactoryModel),
	Notify(String, u32),
	EnablePluginOnSidebar(Plugin),
	AddPluginToSidebar(Plugin),
	DisablePluginOnSidebar(Plugin),
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

#[relm4::component(pub)]
impl Component for App {
	type CommandOutput = ();
	type Input = Event;
	type Output = ();
	type Init = ();
	type Widgets = AppWidgets;

	fn init(
		_init: Self::Init,
		root: &Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let preferences: &str = fl!("preferences");
		let keyboard_shortcuts: &str = fl!("keyboard-shortcuts");
		let about_done: &str = fl!("about-done");
		let quit: &str = fl!("quit");

		let app = main_app();

		app.connect_shutdown(|_| {
			let processes = System::new_all();
			let mut local = processes.processes_by_exact_name("local-plugin");
			if let Some(process) = local.next() {
				if process.kill() {
					info!("The {} process was killed.", process.name());
				} else {
					error!("Failed to kill process.");
				}
			} else {
				info!("Process is not running.");
			}
		});

		let actions = RelmActionGroup::<WindowActionGroup>::new();

		let sidebar_controller = SidebarComponentModel::builder()
			.launch(())
			.forward(sender.input_sender(), |message| match message {
				SidebarComponentOutput::DisablePlugin => Event::DisablePlugin,
				SidebarComponentOutput::ListSelected(list) => {
					Event::TaskListSelected(list)
				},
				SidebarComponentOutput::Forward => Event::Forward,
				SidebarComponentOutput::Notify(msg, timeout) => {
					Event::Notify(msg, timeout)
				},
				SidebarComponentOutput::SelectSmartList(list) => {
					Event::SelectSmartList(list)
				},
			});

		let content_controller = ContentComponentModel::builder()
			.launch(())
			.forward(sender.input_sender(), |message| match message {
				ContentComponentOutput::Notify(msg, timeout) => {
					Event::Notify(msg, timeout)
				},
			});

		let preferences_controller = PreferencesComponentModel::builder()
			.launch(())
			.forward(sender.input_sender(), |message| match message {
				PreferencesComponentOutput::AddPluginToSidebar(plugin) => {
					Event::AddPluginToSidebar(plugin)
				},
				PreferencesComponentOutput::EnablePluginOnSidebar(plugin) => {
					Event::EnablePluginOnSidebar(plugin)
				},
				PreferencesComponentOutput::DisablePluginOnSidebar(plugin) => {
					Event::DisablePluginOnSidebar(plugin)
				},
				PreferencesComponentOutput::ToggleCompact(compact) => {
					Event::ToggleCompact(compact)
				},
			});

		let welcome_controller = WelcomeComponent::builder().launch(()).detach();

		let mut model = App::new(
			sidebar_controller,
			content_controller,
			preferences_controller,
			welcome_controller,
			None,
		);

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

		actions.add_action(&preferences_action);
		actions.add_action(&shortcuts_action);
		actions.add_action(&about_action);
		actions.add_action(&quit_action);

		widgets.main_window.insert_action_group(
			WindowActionGroup::NAME,
			Some(&actions.into_action_group()),
		);

		ComponentParts { model, widgets }
	}

	fn update_with_view(
		&mut self,
		widgets: &mut Self::Widgets,
		message: Self::Input,
		sender: ComponentSender<Self>,
		_root: &Self::Root,
	) {
		match message {
			Event::Quit => main_app().quit(),
			Event::CloseWarning => self.warning_revealed = false,
			Event::TaskListSelected(list) => {
				self.page_title = Some(list.list.name.clone());
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
				set_application: Some(&crate::setup::main_app()),
			},

			add_css_class?: if PROFILE == "Devel" {
				Some("devel")
			} else {
				None
			},

			add_controller = &gtk::GestureClick {
				connect_pressed[sender] => move |_, _, _, _| {
					sender.input(Event::CloseWarning);
				}
			},


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
						append = &gtk::InfoBar {
							set_message_type: gtk::MessageType::Warning,
							#[watch]
							set_revealed: model.warning_revealed,
							gtk::Label {
								set_wrap: true,
								add_css_class: "warning",
								set_text: fl!("alpha-warning")
							}
						},
					}
				}
			}
		}
	}
}

pub fn toast<T: ToString>(title: T, timeout: u32) -> Toast {
	Toast::builder()
		.title(title.to_string().as_str())
		.timeout(timeout)
		.build()
}
