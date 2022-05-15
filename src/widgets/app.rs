use glib::clone;
use glib::Sender;
use relm4::{AppUpdate, Components, Model, RelmComponent, send, Widgets};
use relm4::gtk::prelude::GtkWindowExt;

use crate::{adw, adw::prelude::AdwApplicationWindowExt, gtk, gtk::gio, gtk::prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt}};
use crate::widgets::contents::content::{ContentModel, ContentMsg};
use crate::widgets::panel::sidebar::{SidebarModel, SidebarMsg};
use crate::widgets::panel::theme_selector::ThemeSelector;
use crate::widgets::popovers::new_list::NewListModel;

pub struct AppModel {
    message: Option<AppMsg>,
}

impl AppModel {
    pub fn new() -> Self {
        Self {
            message: None
        }
    }
}

pub enum AppMsg {
    UpdateContent(ContentMsg),
    UpdateSidebar(SidebarMsg),
    Folded,
    Unfolded,
    Forward,
    Back,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

impl AppUpdate for AppModel {
    fn update(
        &mut self,
        msg: Self::Msg,
        components: &Self::Components,
        _sender: Sender<Self::Msg>,
    ) -> bool {
        match msg {
            AppMsg::UpdateContent(msg) => components.content.send(msg).unwrap(),
            AppMsg::UpdateSidebar(msg) => components.sidebar.send(msg).unwrap(),
            _ => self.message = Some(msg),
        }
        true
    }
}

pub struct AppComponents {
    sidebar: RelmComponent<SidebarModel, AppModel>,
    content: RelmComponent<ContentModel, AppModel>,
    new_list: RelmComponent<NewListModel, AppModel>,
    theme_selector: RelmComponent<ThemeSelector, AppModel>,
}

impl Components<AppModel> for AppComponents {
    fn init_components(parent_model: &AppModel, parent_sender: Sender<AppMsg>) -> Self {
        AppComponents {
            sidebar: RelmComponent::new(parent_model, parent_sender.clone()),
            content: RelmComponent::new(parent_model, parent_sender.clone()),
            new_list: RelmComponent::new(parent_model, parent_sender.clone()),
            theme_selector: RelmComponent::new(parent_model, parent_sender),
        }
    }

    fn connect_parent(&mut self, _parent_widgets: &AppWidgets) {}
}

#[relm4::widget(pub)]
impl Widgets<AppModel, ()> for AppWidgets {
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
                                pack_start: new_list_button = &gtk::MenuButton {
                                    set_icon_name: "value-increase-symbolic",
                                    add_css_class: "raised",
                                    set_has_frame: true,
                                    set_direction: gtk::ArrowType::None,
                                    set_popover: Some(components.new_list.root_widget()),
                                },
                            },
                            append: components.sidebar.root_widget()
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
                                        send!(sender, AppMsg::Back);
                                    }
                                },
                            },
                            append: components.content.root_widget()
                        },
                        connect_folded_notify: clone!(@weak content, @strong sender => move |leaflet| {
                            if leaflet.is_folded() {
                                leaflet.set_visible_child(&content);
                                send!(sender, AppMsg::Folded);
                            } else {
                                send!(sender, AppMsg::Unfolded);
                            }
                        })
                    }
                }
            },
        }
    }

    fn post_view() {
        if let Some(msg) = &model.message {
            match msg {
                AppMsg::Folded => {
                    self.leaflet.set_visible_child(&self.content);
                    self.go_back_button.set_visible(true);
                    sidebar_header.set_show_start_title_buttons(true);
                    sidebar_header.set_show_end_title_buttons(true);
                }
                AppMsg::Unfolded => {
                    self.go_back_button.set_visible(false);
                    sidebar_header.set_show_start_title_buttons(false);
                    sidebar_header.set_show_end_title_buttons(false);
                }
                AppMsg::Forward => self.leaflet.set_visible_child(&self.content),
                AppMsg::Back => self.leaflet.set_visible_child(&self.sidebar),
                _ => {}
            }
        }
    }
}