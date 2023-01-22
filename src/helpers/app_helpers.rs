use gtk4::traits::GtkWindowExt;
use relm4::{
	actions::{RelmAction, RelmActionGroup},
	component::{AsyncComponent, AsyncComponentController},
	gtk, AsyncComponentSender, Component, ComponentBuilder,
};

use crate::{
	app::{App, Event},
	application::{info::PROFILE, setup},
	widgets::components::{
		content::{ContentComponentModel, ContentComponentOutput},
		preferences::{PreferencesComponentModel, PreferencesComponentOutput},
		sidebar::{SidebarComponentModel, SidebarComponentOutput},
		welcome::WelcomeComponent,
	},
};

relm4::new_action_group!(pub(super) WindowActionGroup, "win");
relm4::new_stateless_action!(
	PreferencesAction,
	WindowActionGroup,
	"preferences"
);
relm4::new_stateless_action!(pub(super) ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");
relm4::new_stateless_action!(QuitAction, WindowActionGroup, "quit");

impl App {
	pub fn new(sender: AsyncComponentSender<Self>) -> Self {
		let sidebar = SidebarComponentModel::builder().launch(()).forward(
			sender.input_sender(),
			|message| match message {
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
			},
		);

		let content = ContentComponentModel::builder().launch(()).forward(
			sender.input_sender(),
			|message| match message {
				ContentComponentOutput::Notify(msg, timeout) => {
					Event::Notify(msg, timeout)
				},
			},
		);

		let preferences = PreferencesComponentModel::builder().launch(()).forward(
			sender.input_sender(),
			|message| match message {
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
			},
		);

		let welcome = WelcomeComponent::builder().launch(()).detach();

		Self {
			sidebar,
			content,
			preferences,
			welcome,
			about_dialog: None,
			page_title: None,
			warning_revealed: PROFILE == "Devel",
		}
	}

	pub async fn init_services(self) -> Self {
		match setup::init_app() {
			Ok(_) => (),
			Err(_) => panic!("Failed to initialize the application."),
		}
		match setup::init_services().await {
			Ok(_) => (),
			Err(_) => panic!("Failed to initialize services."),
		};
		self
	}
}
