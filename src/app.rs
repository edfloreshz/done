use crate::application::info::PROFILE;
use crate::application::plugin::Plugin;
use crate::application::setup::{self, main_app};
use crate::factories::task_list::model::TaskListFactoryModel;
use crate::fl;
use crate::widgets::about_dialog::AboutDialog;
use crate::widgets::content::messages::{ContentInput, ContentOutput};
use crate::widgets::content::model::ContentModel;
use crate::widgets::lists::messages::{TaskListsInput, TaskListsOutput};
use crate::widgets::lists::model::TaskListsModel;
use crate::widgets::preferences::messages::PreferencesComponentOutput;
use crate::widgets::preferences::model::PreferencesComponentModel;
use crate::widgets::sidebar::messages::{
	SidebarComponentInput, SidebarComponentOutput,
};
use crate::widgets::sidebar::model::SidebarComponentModel;
use crate::widgets::smart_lists::sidebar::model::SmartList;
use crate::widgets::welcome::WelcomeComponent;
use gtk::prelude::*;
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

pub struct App {
	sidebar: AsyncController<SidebarComponentModel>,
	content: AsyncController<ContentModel>,
	preferences: AsyncController<PreferencesComponentModel>,
	task_lists: AsyncController<TaskListsModel>,
	welcome: Controller<WelcomeComponent>,
	about_dialog: Option<Controller<AboutDialog>>,
	page_title: Option<String>,
	warning_revealed: bool,
}

impl App {
	pub fn new(
		sidebar: AsyncController<SidebarComponentModel>,
		content: AsyncController<ContentModel>,
		preferences: AsyncController<PreferencesComponentModel>,
		task_lists: AsyncController<TaskListsModel>,
		welcome: Controller<WelcomeComponent>,
		about_dialog: Option<Controller<AboutDialog>>,
	) -> Self {
		Self {
			sidebar,
			content,
			preferences,
			task_lists,
			welcome,
			about_dialog,
			page_title: None,
			warning_revealed: true,
		}
	}
}

#[derive(Debug)]
pub enum Event {
	TaskListSelected(TaskListFactoryModel),
	Notify(String, u32),
	PluginSelected(Plugin),
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

	view! {
		#[root]
		main_window = adw::ApplicationWindow::new(&main_app()) {
			set_default_width: 800,
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

			gtk::Box {
				set_orientation: gtk::Orientation::Vertical,
				#[name(overlay)]
				adw::ToastOverlay {
					#[wrap(Some)]
					set_child: leaflet = &adw::Leaflet {
						set_can_navigate_back: true,
						append: sidebar = &gtk::Box {
							set_width_request: 300,
							adw::Leaflet {
								gtk::Box {
									set_css_classes: &["view"],
									set_orientation: gtk::Orientation::Vertical,
									set_width_request: 20,
									#[name = "services_sidebar_header"]
									gtk::CenterBox {
										set_height_request: 46,
										#[wrap(Some)]
										set_center_widget = &gtk::MenuButton {
											set_valign: gtk::Align::Center,
											set_css_classes: &["flat"],
											set_icon_name: "open-menu-symbolic",
											set_menu_model: Some(&primary_menu),
										},
									},
									gtk::Separator::default(),
									append: model.sidebar.widget(),
								},
								gtk::Separator::default(),
								append: model.task_lists.widget(),
							},
						},
						append: &gtk::Separator::default(),
						#[name(content)]
						append = &gtk::Box {
							set_orientation: gtk::Orientation::Vertical,
							#[name = "content_header"]
							append = &adw::HeaderBar {
								set_hexpand: true,
								set_css_classes: &["flat"],
								set_show_start_title_buttons: false,
								set_show_end_title_buttons: true,
								#[watch]
								set_title_widget: Some(&gtk::Label::new(model.page_title.as_deref())),
								pack_start = &gtk::Button {
									set_icon_name: "file-search-symbolic",
								},
								pack_start: go_back_button = &gtk::Button {
									set_icon_name: "go-previous-symbolic",
									set_visible: false,
									connect_clicked[sender] => move |_| {
										sender.input(Event::Back);
									}
								},
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
			}
		}
	}

	fn init_loading_widgets(root: &mut Self::Root) -> Option<LoadingWidgets> {
		view! {
				#[local_ref]
				root {
					set_title: Some("Done"),
					set_default_size: (800, 700),

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

		let preferences: &str = fl!("preferences");
		let keyboard_shortcuts: &str = fl!("keyboard-shortcuts");
		let about_done: &str = fl!("about-done");
		let quit: &str = fl!("quit");

		let actions = RelmActionGroup::<WindowActionGroup>::new();

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
				PreferencesComponentOutput::RemovePluginFromSidebar(plugin) => {
					Event::RemovePluginFromSidebar(plugin)
				},
			});

		let sidebar_controller = SidebarComponentModel::builder()
			.launch(())
			.forward(sender.input_sender(), |message| match message {
				SidebarComponentOutput::PluginSelected(plugin) => {
					Event::PluginSelected(plugin)
				},
				SidebarComponentOutput::DisablePlugin => Event::DisablePlugin,
				SidebarComponentOutput::ListSelected(list) => {
					Event::TaskListSelected(*list)
				},
				SidebarComponentOutput::Forward => Event::Forward,
				SidebarComponentOutput::Notify(msg, timeout) => {
					Event::Notify(msg, timeout)
				},
				SidebarComponentOutput::SelectSmartList(list) => {
					Event::SelectSmartList(list)
				},
			});

		let content_controller = ContentModel::builder().launch(()).forward(
			sender.input_sender(),
			|message| match message {
				ContentOutput::Notify(msg, timeout) => Event::Notify(msg, timeout),
			},
		);

		let task_lists_controller = TaskListsModel::builder().launch(None).forward(
			sender.input_sender(),
			|message| match message {
				TaskListsOutput::Forward => Event::Forward,
				TaskListsOutput::Notify(_) => todo!(),
				TaskListsOutput::ListSelected(task_list) => {
					Event::TaskListSelected(*task_list)
				},
			},
		);

		let welcome_controller = WelcomeComponent::builder().launch(()).detach();

		let mut model = App::new(
			sidebar_controller,
			content_controller,
			preferences_controller,
			task_lists_controller,
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
				let processes = System::new_all();
				for plugin in Plugin::get_local().unwrap() {
					let mut plugin =
						processes.processes_by_exact_name(&plugin.process_name);
					if let Some(process) = plugin.next() {
						if process.kill() {
							tracing::info!("The {} process was killed.", process.name());
						} else {
							tracing::error!("Failed to kill process.");
						}
					} else {
						tracing::info!("Process is not running.");
					}
				}
				main_app().quit()
			},
			Event::CloseWarning => self.warning_revealed = false,
			Event::PluginSelected(plugin) => self
				.task_lists
				.sender()
				.send(TaskListsInput::PluginSelected(plugin))
				.unwrap_or_default(),
			Event::TaskListSelected(list) => {
				self.page_title = Some(list.list.name.clone());
				self
					.content
					.sender()
					.send(ContentInput::TaskListSelected(list))
					.unwrap_or_default();
			},
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
			Event::Folded => {
				if self.page_title.is_some() {
					widgets.leaflet.set_visible_child(&widgets.content);
				} else {
					widgets.leaflet.set_visible_child(&widgets.sidebar);
				}
				widgets.go_back_button.set_visible(true);
				// widgets.sidebar_header.set_show_start_title_buttons(true);
				// widgets.sidebar_header.set_show_end_title_buttons(true);
			},
			Event::Unfolded => {
				widgets.go_back_button.set_visible(false);
				// widgets.sidebar_header.set_show_start_title_buttons(false);
				// widgets.sidebar_header.set_show_end_title_buttons(false);
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
					.send(ContentInput::DisablePlugin)
					.unwrap_or_default()
			},
			Event::SelectSmartList(list) => {
				self.page_title = Some(list.name());
				self
					.task_lists
					.sender()
					.send(TaskListsInput::SmartListSelected)
					.unwrap_or_default();
				self
					.content
					.sender()
					.send(ContentInput::SelectSmartList(list))
					.unwrap_or_default();
			},
			Event::ToggleCompact(compact) => self
				.content
				.sender()
				.send(ContentInput::ToggleCompact(compact))
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
}

pub fn toast<T: ToString>(title: T, timeout: u32) -> Toast {
	Toast::builder()
		.title(title.to_string().as_str())
		.timeout(timeout)
		.build()
}
