use gtk::prelude::*;
use relm4::{
	actions::{ActionGroupName, RelmAction, RelmActionGroup},
	adw, gtk, Component, ComponentBuilder, ComponentController, ComponentParts,
	ComponentSender, Controller, SimpleComponent,
};

use crate::main_app;
use crate::widgets::modals::about::AboutDialog;
use crate::widgets::popover::new_list::NewListOutput;
use crate::{
	config::PROFILE,
	widgets::{
		component::{
			content::{ContentInput, ContentModel, ContentOutput},
			sidebar::{SidebarInput, SidebarModel, SidebarOutput},
		},
		factory::list::ListType,
		popover::new_list::NewListModel,
	},
};
use crate::{
	data::{
		models::generic::lists::GenericList, plugins::Plugins,
		traits::provider::Provider,
	},
	SERVICES,
};

pub(super) struct App {
	message: Option<AppMsg>,
	sidebar: Controller<SidebarModel>,
	content: Controller<ContentModel>,
	_new_list_popover: Controller<NewListModel>,
	about_dialog: Controller<AboutDialog>,
}

impl App {
	pub fn new(
		sidebar: Controller<SidebarModel>,
		content: Controller<ContentModel>,
		new_list_popover: Controller<NewListModel>,
		about_dialog: Controller<AboutDialog>,
	) -> Self {
		Self {
			message: None,
			sidebar,
			content,
			_new_list_popover: new_list_popover,
			about_dialog,
		}
	}
}

#[derive(Debug)]
pub(super) enum AppMsg {
	AddTaskList(String, String),
	ListSelected(usize, String, GenericList),
	UpdateSidebarCounters(Vec<ListType>),
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
	type InitParams = Option<Vec<Box<dyn Provider>>>;

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
							)
							.object::<gtk::ShortcutsWindow>("help_overlay")
							.unwrap() -> gtk::ShortcutsWindow {
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
							#[wrap(Some)]
							set_title_widget = &gtk::Box {
								set_orientation: gtk::Orientation::Horizontal,
								set_spacing: 5,
								gtk::Image {
									set_from_resource: Some("/dev/edfloreshz/Done/icons/scalable/apps/app-icon.svg"),
								},
								gtk::Label {
									set_text: "Done"
								}
							},
							pack_start: new_list_button = &gtk::MenuButton {
								set_icon_name: "value-increase-symbolic",
								add_css_class: "raised",
								set_has_frame: true,
								set_popover: Some(new_list_controller.widget())
							},
							pack_end = &gtk::MenuButton {
								set_icon_name: "open-menu-symbolic",
								set_menu_model: Some(&primary_menu),
							}
						},
						append: sidebar_controller.widget()
					},
					append: &gtk::Separator::default(),
					append: content = &gtk::Box {
						set_orientation: gtk::Orientation::Vertical,
						set_hexpand: true,
						set_vexpand: true,
						append = &adw::HeaderBar {
							set_hexpand: true,
							set_show_end_title_buttons: true,
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
		_params: Self::InitParams,
		root: &Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let plugins = Plugins::init();

		if plugins.local.is_enabled() {
			unsafe {
				SERVICES
					.get_mut()
					.unwrap()
					.push(Box::new(plugins.local.clone()))
			}
		}

		let sidebar_controller =
			SidebarModel::builder()
				.launch(None)
				.forward(&sender.input, |message| match message {
					SidebarOutput::ListSelected(index, provider, list) => {
						AppMsg::ListSelected(index, provider, list)
					},
					SidebarOutput::Forward => AppMsg::Forward,
				});
		let content_controller =
			ContentModel::builder()
				.launch(None)
				.forward(&sender.input, |message| match message {
					ContentOutput::UpdateCounters(lists) => {
						AppMsg::UpdateSidebarCounters(lists)
					},
				});
		let new_list_controller = NewListModel::builder()
			.launch(Some("local".to_string()))
			.forward(&sender.input, |message| match message {
				NewListOutput::AddTaskListToSidebar(provider, name) => {
					AppMsg::AddTaskList(provider, name)
				},
			});

		let widgets = view_output!();

		let about_dialog = ComponentBuilder::new()
			.launch(widgets.main_window.upcast_ref::<gtk::Window>().clone())
			.detach();

		let actions = RelmActionGroup::<WindowActionGroup>::new();

		let shortcuts_action = {
			let shortcuts = widgets.shortcuts.clone();
			RelmAction::<ShortcutsAction>::new_stateless(move |_| {
				shortcuts.present();
			})
		};

		let model = App::new(
			sidebar_controller,
			content_controller,
			new_list_controller,
			about_dialog,
		);

		let about_action = {
			let sender = model.about_dialog.sender().clone();
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
			AppMsg::AddTaskList(provider, title) => self
				.sidebar
				.sender()
				.send(SidebarInput::AddTaskList(provider, title)),
			AppMsg::ListSelected(index, provider, list) => self
				.content
				.sender()
				.send(ContentInput::SetTaskList(index, provider, list)),
			_ => self.message = Some(message),
		}
	}
}
