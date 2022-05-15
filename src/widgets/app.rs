use relm4::{
    Component, ComponentController, ComponentParts, ComponentSender, Controller, SimpleComponent,
};
use relm4::gtk::prelude::GtkWindowExt;

use crate::{
    adw,
    adw::prelude::AdwApplicationWindowExt,
    gtk,
    gtk::gio,
    gtk::prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt},
};
use crate::models::list::List;
use crate::widgets::component::content::{ContentInput, ContentModel, ContentOutput};
use crate::widgets::component::sidebar::{SidebarInput, SidebarModel, SidebarOutput};
use crate::widgets::factory::list::ListType;
use crate::widgets::popover::new_list::{NewListModel, NewListOutput};

pub struct AppModel {
    message: Option<Input>,
    sidebar: Controller<SidebarModel>,
    content: Controller<ContentModel>,
    new_list_popover: Controller<NewListModel>,
}

impl AppModel {
    pub fn new(sidebar: Controller<SidebarModel>, content: Controller<ContentModel>, new_list_popover: Controller<NewListModel>) -> Self {
        Self {
            message: None,
            sidebar,
            content,
            new_list_popover,
        }
    }
}

pub enum Input {
    AddList(String),
    ListSelected(usize, List),
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
        window = adw::ApplicationWindow {
            set_default_width: 800,
            set_default_height: 700,
            set_width_request: 300,
            set_height_request: 300,

            set_content: overlay = Some(&gtk::Overlay) {
                set_child: stack = Some(&gtk::Stack) {
                    set_hexpand: true,
                    set_vexpand: true,
                    set_transition_duration: 250,
                    set_transition_type: gtk::StackTransitionType::Crossfade,
                    add_child: leaflet = &adw::Leaflet {
                        set_can_navigate_back: true,
                        append: sidebar = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_width_request: 280,
                            append: sidebar_header = &adw::HeaderBar {
                                set_show_end_title_buttons: false,
                                set_title_widget = Some(&gtk::Box) {
                                    set_orientation: gtk::Orientation::Horizontal,
                                    set_spacing: 5,
                                    append = &gtk::Image {
                                        set_icon_name: Some("dev.edfloreshz.Done")
                                    },
                                    append = &gtk::Label {
                                        set_text: "Done"
                                    }
                                },
                                pack_end: options_button = &gtk::MenuButton {
                                    set_icon_name: "open-menu-symbolic",
                                    add_css_class: "flat",
                                    set_has_frame: true,
                                    set_direction: gtk::ArrowType::None,
                                    set_popover: popover = Some(&gtk::PopoverMenu::from_model(None::<&gio::MenuModel>)) {
                                        // TODO: Figure out a way to include the theme selector in the menu.
                                        // add_child: args!(component.theme_selector.root_widget(), ""),
                                        set_menu_model = Some(&gio::Menu) {
                                            append: args!(Some("About"), Some("app.about")),
                                            append: args!(Some("Quit"), Some("app.quit"))
                                        },
                                    },
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
                                    connect_clicked(sender) => move |_| {
                                        sender.input.send(Input::Back);
                                    }
                                },
                            },
                            append: model.content.widget()
                        },
                        connect_folded_notify(sender) => move |leaflet| {
                            if leaflet.is_folded() {
                                sender.input.send(Input::Folded);
                            } else {
                                sender.input.send(Input::Unfolded);
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
                }
                Input::Unfolded => {
                    go_back_button.set_visible(false);
                    sidebar_header.set_show_start_title_buttons(false);
                    sidebar_header.set_show_end_title_buttons(false);
                }
                Input::Forward => leaflet.set_visible_child(content),
                Input::Back => leaflet.set_visible_child(sidebar),
                _ => {}
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
                    SidebarOutput::ListSelected(index, list) => Input::ListSelected(index, list),
                    SidebarOutput::Forward => Input::Forward
                }),
            ContentModel::builder()
                .launch(None)
                .forward(&sender.input, |message| match message {
                    ContentOutput::UpdateCounters(lists) => Input::UpdateSidebarCounters(lists)
                }),
            NewListModel::builder()
                .launch(())
                .forward(&sender.input, |message| match message {
                    NewListOutput::AddNewList(name) => Input::AddList(name)
                }),
        );
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: &ComponentSender<Self>) {
        match message {
            Input::AddList(title) => self.sidebar.sender().send(SidebarInput::AddList(title)),
            Input::ListSelected(index, list) => self
                .content
                .sender()
                .send(ContentInput::SetTaskList(index, list)),
            Input::UpdateSidebarCounters(lists) => self.sidebar.sender().send(SidebarInput::UpdateCounters(lists)),
            _ => self.message = Some(message)
        }
    }
}
