use crate::application::plugin::Plugin;
use crate::config::PROFILE;
use crate::fl;
use crate::setup::main_app;
use crate::widgets::components::content::{
	ContentInput, ContentModel, ContentOutput,
};
use crate::widgets::components::preferences::{Preferences, PreferencesOutput};
use crate::widgets::components::sidebar::{
	SidebarInput, SidebarModel, SidebarOutput,
};
use crate::widgets::components::welcome::Welcome;
use crate::widgets::factory::list::ListData;
use crate::widgets::modals::about::AboutDialog;
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

pub struct App {
	sidebar: AsyncController<SidebarModel>,
	content: AsyncController<ContentModel>,
	preferences: Controller<Preferences>,
	welcome: Controller<Welcome>,
	about_dialog: Option<Controller<AboutDialog>>,
	page_title: Option<String>,
	warning_revealed: bool,
}

impl App {
	pub fn new(
		sidebar: AsyncController<SidebarModel>,
		content: AsyncController<ContentModel>,
		preferences: Controller<Preferences>,
		welcome: Controller<Welcome>,
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
pub enum AppMsg {
	TaskListSelected(ListData),
	Notify(String),
	EnablePluginOnSidebar(Plugin),
	DisablePluginOnSidebar(Plugin),
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
	type Input = AppMsg;
	type Output = ();
	type Init = ();
	type Widgets = AppWidgets;

	menu! {
		primary_menu: {
			section! {
				"_Preferences" => PreferencesAction,
				"_Keyboard" => ShortcutsAction,
				"_About Done" => AboutAction,
				"_Quit" => QuitAction,
			}
		}
	}

	view! {
		#[root]
		main_window = adw::ApplicationWindow::new(&main_app()) {
			set_default_width: 800,
			set_default_height: 700,
			connect_close_request[sender] => move |_| {
				sender.input(AppMsg::Quit);
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
					sender.input(AppMsg::CloseWarning)
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
											sender.input(AppMsg::Back);
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
									sender.input(AppMsg::Folded);
								} else {
									sender.input(AppMsg::Unfolded);
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

	fn init(
		_init: Self::Init,
		root: &Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let actions = RelmActionGroup::<WindowActionGroup>::new();

		let sidebar_controller = SidebarModel::builder().launch(()).forward(
			sender.input_sender(),
			|message| match message {
				SidebarOutput::ListSelected(list) => AppMsg::TaskListSelected(list),
				SidebarOutput::Forward => AppMsg::Forward,
				SidebarOutput::Notify(msg) => AppMsg::Notify(msg),
			},
		);

		let content_controller = ContentModel::builder().launch(()).forward(
			sender.input_sender(),
			|message| match message {
				ContentOutput::Notify(msg) => AppMsg::Notify(msg),
			},
		);

		let preferences_controller = Preferences::builder().launch(()).forward(
			sender.input_sender(),
			|message| match message {
				PreferencesOutput::EnablePluginOnSidebar(plugin) => {
					AppMsg::EnablePluginOnSidebar(plugin)
				},
				PreferencesOutput::DisablePluginOnSidebar(plugin) => {
					AppMsg::DisablePluginOnSidebar(plugin)
				},
			},
		);

		let welcome_controller = Welcome::builder().launch(()).detach();

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
				sender.input_sender().send(AppMsg::Quit).unwrap_or_default();
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
			AppMsg::Quit => main_app().quit(),
			AppMsg::CloseWarning => self.warning_revealed = false,
			AppMsg::TaskListSelected(list) => {
				self.page_title = Some(list.data.name.clone());
				self
					.content
					.sender()
					.send(ContentInput::TaskListSelected(list))
					.unwrap_or_default();
			},
			AppMsg::Notify(msg) => widgets.overlay.add_toast(&toast(msg)),
			AppMsg::Folded => {
				if self.page_title.is_some() {
					widgets.leaflet.set_visible_child(&widgets.content);
				} else {
					widgets.leaflet.set_visible_child(&widgets.sidebar);
				}
				widgets.go_back_button.set_visible(true);
				widgets.sidebar_header.set_show_start_title_buttons(true);
				widgets.sidebar_header.set_show_end_title_buttons(true);
			},
			AppMsg::Unfolded => {
				widgets.go_back_button.set_visible(false);
				widgets.sidebar_header.set_show_start_title_buttons(false);
				widgets.sidebar_header.set_show_end_title_buttons(false);
			},
			AppMsg::Forward => widgets.leaflet.set_visible_child(&widgets.content),
			AppMsg::Back => widgets.leaflet.set_visible_child(&widgets.sidebar),
			AppMsg::EnablePluginOnSidebar(plugin) => self
				.sidebar
				.sender()
				.send(SidebarInput::EnableService(plugin))
				.unwrap_or_default(),
			AppMsg::DisablePluginOnSidebar(plugin) => self
				.sidebar
				.sender()
				.send(SidebarInput::DisableService(plugin))
				.unwrap_or_default(),
		}
		self.update_view(widgets, sender)
	}
}

pub fn toast(title: impl ToString) -> Toast {
	Toast::builder()
		.title(title.to_string().as_str())
		.timeout(1)
		.build()
}
