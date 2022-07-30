use relm4::gtk::prelude::GtkWindowExt;
use relm4::{
	Component, ComponentController, ComponentParts, ComponentSender, Controller,
	SimpleComponent,
};

use crate::widgets::component::content::{
	ContentInput, ContentModel, ContentOutput,
};
use crate::widgets::component::sidebar::{
	SidebarInput, SidebarModel, SidebarOutput,
};
use crate::widgets::factory::list::ListType;
use crate::widgets::popover::main_menu::MainMenuInput;
use crate::widgets::popover::new_list::{NewListModel, NewListOutput};
use crate::{
	adw, gtk,
	gtk::prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt},
};
use crate::core::models::generic::lists::GenericList;

pub struct AppModel {
	message: Option<Input>,
	sidebar: Controller<SidebarModel>,
	content: Controller<ContentModel>,
	new_list_popover: Controller<NewListModel>,
	main_menu_popover: Controller<MainMenuInput>,
}

impl AppModel {
	pub fn new(
		sidebar: Controller<SidebarModel>,
		content: Controller<ContentModel>,
		new_list_popover: Controller<NewListModel>,
		main_menu_popover: Controller<MainMenuInput>,
	) -> Self {
		Self {
			message: None,
			sidebar,
			content,
			new_list_popover,
			main_menu_popover,
		}
	}
}

#[derive(Debug)]
pub enum Input {
	AddList(String, String),
	ListSelected(usize, String, GenericList),
	UpdateSidebarCounters(Vec<ListType>),
	Folded,
	Unfolded,
	Forward,
	Back,
}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
	type Input = Input;
	type Output = ();
	type InitParams = Option<Input>;
	type Widgets = AppWidgets;

	view! {
		#[root]
		window = adw::ApplicationWindow {
			set_default_width: 800,
			set_default_height: 700,
			set_width_request: 300,
			set_height_request: 300,
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
								pack_end: options_button = &gtk::MenuButton {
									set_icon_name: "open-menu-symbolic",
									add_css_class: "flat",
									set_has_frame: true,
									set_direction: gtk::ArrowType::None,
									set_popover: Some(model.main_menu_popover.widget()),
								},
								pack_start: new_list_button = &gtk::MenuButton {
									set_icon_name: "value-increase-symbolic",
									add_css_class: "raised",
									set_has_frame: true,
									set_popover: Some(model.new_list_popover.widget())
								},
							},
							append: model.sidebar.widget()
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
										sender.input(Input::Back);
									}
								},
							},
							append: model.content.widget()
						},
						connect_folded_notify[sender] => move |leaflet| {
							if leaflet.is_folded() {
								sender.input(Input::Folded);
							} else {
								sender.input(Input::Unfolded);
							}
						}
					}
				}
			},
		}
	}

	fn post_view() {
		if let Some(msg) = &model.message {
			match msg {
				Input::Folded => {
					leaflet.set_visible_child(content);
					go_back_button.set_visible(true);
					sidebar_header.set_show_start_title_buttons(true);
					sidebar_header.set_show_end_title_buttons(true);
				},
				Input::Unfolded => {
					go_back_button.set_visible(false);
					sidebar_header.set_show_start_title_buttons(false);
					sidebar_header.set_show_end_title_buttons(false);
				},
				Input::Forward => leaflet.set_visible_child(content),
				Input::Back => leaflet.set_visible_child(sidebar),
				_ => {},
			}
		}
	}

	fn init(
		_params: Self::InitParams,
		root: &Self::Root,
		sender: &ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = AppModel::new(
			SidebarModel::builder()
				.launch(None)
				.forward(&sender.input, |message| match message {
					SidebarOutput::ListSelected(index, provider,  list) => {
						Input::ListSelected(index, provider, list)
					},
					SidebarOutput::Forward => Input::Forward,
				}),
			ContentModel::builder()
				.launch(None)
				.forward(&sender.input, |message| match message {
					ContentOutput::UpdateCounters(lists) => {
						Input::UpdateSidebarCounters(lists)
					},
				}),
			NewListModel::builder()
				.launch(())
				.forward(&sender.input, |message| match message {
					NewListOutput::AddNewList(provider, name) => Input::AddList(provider, name),
				}),
			MainMenuInput::builder().launch(()).detach(),
		);
		let widgets = view_output!();
		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: &ComponentSender<Self>) {
		match message {
			Input::AddList(provider, title) => {
				self.sidebar.sender().send(SidebarInput::AddList(provider, title))
			},
			Input::ListSelected(index, provider, list) => self
				.content
				.sender()
				.send(ContentInput::SetTaskList(index, provider,list)),
			Input::UpdateSidebarCounters(lists) => self
				.sidebar
				.sender()
				.send(SidebarInput::UpdateCounters(lists)),
			_ => self.message = Some(message),
		}
	}
}
