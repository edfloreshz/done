use relm4::{Component, ComponentController, ComponentParts, ComponentSender, Controller, SimpleComponent};
use relm4::gtk::prelude::GtkWindowExt;

use crate::{adw, adw::prelude::AdwApplicationWindowExt, gtk, gtk::gio, gtk::prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt}};
use crate::widgets::sidebar::{SidebarInput, SidebarModel, SidebarOutput};

pub struct AppModel {
    message: Option<Input>,
    sidebar: Controller<SidebarModel>,
}

impl AppModel {
    pub(crate) fn new() -> Self {
        Self {
            message: None,
            sidebar: SidebarModel::builder()
                .launch(None)
                .connect_receiver(move |sender, message| match message {
                    SidebarOutput::ListSelected(index) => println!("Current index: {}", index)
                }),
        }
    }
}

pub enum Input {
    AddList(String),
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
                                        // add_child: args!(components.theme_selector.root_widget(), ""),
                                        set_menu_model = Some(&gio::Menu) {
                                            append: args!(Some("About"), Some("app.about")),
                                            append: args!(Some("Quit"), Some("app.quit"))
                                        },
                                    },
                                },
                                pack_start: new_list_button = &gtk::Button {
                                    set_icon_name: "value-increase-symbolic",
                                    add_css_class: "raised",
                                    set_has_frame: true,
                                    connect_clicked(sender) => move |_| {
                                        sender.input(Input::AddList("Test".to_string()));
                                    }
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
                                        // sender.send(Input::Back);
                                    }
                                },
                            },
                            // append: components.content.root_widget()
                        },
                        // connect_folded_notify: clone!(@weak content, @strong sender => move |leaflet| {
                        //     if leaflet.is_folded() {
                        //         leaflet.set_visible_child(&content);
                        //         send!(sender, Input::Folded);
                        //     } else {
                        //         send!(sender, Input::Unfolded);
                        //     }
                        // })
                    }
                }
            },
        }
    }

    fn post_view() {
        println!("Test");
        if let Some(msg) = &model.message {
            // match msg {
            //     AppMsg::Folded => {
            //         self.leaflet.set_visible_child(&self.content);
            //         self.go_back_button.set_visible(true);
            //         sidebar_header.set_show_start_title_buttons(true);
            //         sidebar_header.set_show_end_title_buttons(true);
            //     }
            //     AppMsg::Unfolded => {
            //         self.go_back_button.set_visible(false);
            //         sidebar_header.set_show_start_title_buttons(false);
            //         sidebar_header.set_show_end_title_buttons(false);
            //     }
            //     AppMsg::Forward => self.leaflet.set_visible_child(&self.content),
            //     AppMsg::Back => self.leaflet.set_visible_child(&self.sidebar),
            //     _ => {}
            // }
        }
    }

    fn init(params: Self::InitParams, root: &Self::Root, sender: &ComponentSender<Self>) -> ComponentParts<Self> {
        let model = AppModel::new();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: &ComponentSender<Self>) {
        match message {
            Input::AddList(title) => self.sidebar.sender().send(SidebarInput::AddList(title)),
            _ => self.message = Some(message),
        }
    }
}