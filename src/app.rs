use gtk::prelude::*;
use relm4::{
	actions::{ActionGroupName, RelmAction, RelmActionGroup},
	adw, gtk, Component, ComponentBuilder, ComponentController, ComponentParts,
	ComponentSender, Controller, SimpleComponent,
};

use crate::data::{
	models::generic::lists::GenericTaskList, traits::provider::Provider,
};
use crate::main_app;
use crate::widgets::modals::about::AboutDialog;
use crate::{
	config::PROFILE,
	widgets::components::{
		content::{ContentInput, ContentModel},
		sidebar::{SidebarModel, SidebarOutput},
	},
};

pub(super) struct App {
	message: Option<AppMsg>,
	content: Option<Controller<ContentModel>>,
	about_dialog: Option<Controller<AboutDialog>>,
	content_title: String,
}

impl App {
	pub fn new(
		content: Option<Controller<ContentModel>>,
		about_dialog: Option<Controller<AboutDialog>>,
	) -> Self {
		Self {
			message: None,
			content,
			about_dialog,
			content_title: "All".to_string(),
		}
	}
}
#[allow(dead_code)]
#[derive(Debug)]
pub(super) enum AppMsg {
	ListSelected(GenericTaskList),
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
impl SimpleComponent for App {
	type Input = AppMsg;
	type Output = ();
	type Widgets = AppWidgets;
	type Init = Option<Vec<Box<dyn Provider>>>;

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
			set_width_request: 300,
			set_height_request: 300,
			connect_close_request[sender] => move |_| {
				sender.input(AppMsg::Quit);
				gtk::Inhibit(true)
			},

			#[wrap(Some)]
			set_help_overlay: shortcuts = &gtk::Builder::from_resource(
					"/dev/edfloreshz/Done/ui/gtk/help-overlay.ui"
			).object::<gtk::ShortcutsWindow>("help_overlay").unwrap() -> gtk::ShortcutsWindow {
				set_transient_for: Some(&main_window),
				set_application: Some(&crate::main_app()),
			},

			add_css_class?: if PROFILE == "Devel" {
				Some("devel")
			} else {
				None
			},

			#[name = "overlay"]
			gtk::Overlay {
				#[wrap(Some)]
				set_child: stack = &gtk::Stack {
					set_hexpand: true,
					set_vexpand: true,
					set_transition_duration: 250,
					set_transition_type: gtk::StackTransitionType::Crossfade,
					add_child: leaflet = &adw::Leaflet {
						set_can_navigate_back: true,
						append: sidebar = &gtk::Box {
							set_orientation: gtk::Orientation::Vertical,
							set_width_request: 280,
							#[name = "sidebar_header"]
							adw::HeaderBar {
								set_show_end_title_buttons: false,
								pack_end = &gtk::MenuButton {
									set_icon_name: "open-menu-symbolic",
									set_menu_model: Some(&primary_menu),
								},
							},
							append: sidebar_controller.widget()
						},
						append: &gtk::Separator::default(),
						append: content = &gtk::Box {
							set_orientation: gtk::Orientation::Vertical,
							set_hexpand: true,
							set_vexpand: true,
								#[name = "content_header"]
							append = &adw::HeaderBar {
								set_hexpand: true,
								set_show_start_title_buttons: true,
								#[watch]
								set_title_widget: Some(&gtk::Label::new(Some(&model.content_title))),
								pack_start: go_back_button = &gtk::Button {
									set_icon_name: "go-previous-symbolic",
									set_visible: false,
									connect_clicked[sender] => move |_| {
										sender.input(AppMsg::Back);
									}
								}
							},
							append: content_controller.widget()
						},
						connect_folded_notify[sender] => move |leaflet| {
							if leaflet.is_folded() {
								sender.input(AppMsg::Folded);
							} else {
								sender.input(AppMsg::Unfolded);
							}
						}
					}
				}
			}
		}
	}

	fn post_view() {
		if let Some(msg) = &model.message {
			match msg {
				AppMsg::Folded => {
					leaflet.set_visible_child(content);
					go_back_button.set_visible(true);
					sidebar_header.set_show_start_title_buttons(true);
					sidebar_header.set_show_end_title_buttons(true);
				},
				AppMsg::Unfolded => {
					go_back_button.set_visible(false);
					sidebar_header.set_show_start_title_buttons(false);
					sidebar_header.set_show_end_title_buttons(false);
				},
				AppMsg::Forward => leaflet.set_visible_child(content),
				AppMsg::Back => leaflet.set_visible_child(sidebar),
				_ => {},
			}
		}
	}

	fn init(
		_init: Self::Init,
		root: &Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let sidebar_controller =
			SidebarModel::builder()
				.launch(None)
				.forward(&sender.input, |message| match message {
					SidebarOutput::ListSelected(list) => AppMsg::ListSelected(list),
					SidebarOutput::Forward => AppMsg::Forward,
				});

		let content_controller = ContentModel::builder()
			.launch(None)
			.forward(&sender.input, |message| match message {});

		let actions = RelmActionGroup::<WindowActionGroup>::new();

		let model = App::new(None, None);

		let widgets = view_output!();

		let shortcuts_action = {
			let shortcuts = widgets.shortcuts.clone();
			RelmAction::<ShortcutsAction>::new_stateless(move |_| {
				shortcuts.present();
			})
		};

		let about_dialog = ComponentBuilder::default()
			.launch(widgets.main_window.upcast_ref::<gtk::Window>().clone())
			.detach();

		let model = App::new(Some(content_controller), Some(about_dialog));

		let about_action = {
			let sender = model.about_dialog.as_ref().unwrap().sender().clone();
			RelmAction::<AboutAction>::new_stateless(move |_| {
				sender.send(());
			})
		};

		let quit_action = {
			RelmAction::<QuitAction>::new_stateless(move |_| {
				sender.input_sender().send(AppMsg::Quit)
			})
		};

		actions.add_action(shortcuts_action);
		actions.add_action(about_action);
		actions.add_action(quit_action);

		widgets.main_window.insert_action_group(
			WindowActionGroup::NAME,
			Some(&actions.into_action_group()),
		);

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		match message {
			AppMsg::Quit => main_app().quit(),
			AppMsg::ListSelected(list) => {
				self.content_title = list.display_name.clone();
				self
					.content
					.as_ref()
					.unwrap()
					.sender()
					.send(ContentInput::SetTaskList(list))
			},
			_ => self.message = Some(message),
		}
	}
}
