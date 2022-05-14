use glib::Sender;
use relm4::{send, Widgets, Components, ComponentUpdate, Model, RelmComponent};
use crate::AppModel;
use crate::widgets::contents::content::{ContentModel, ContentMsg};
use crate::widgets::panel::sidebar::{SidebarModel, SidebarMsg};
use crate::{adw, gtk, gtk::prelude::{
    BoxExt, ButtonExt, OrientableExt, PopoverExt, WidgetExt,
}};
use crate::widgets::app::AppMsg;
use glib::clone;
use crate::widgets::panel::theme_selector::ThemeSelector;
use crate::widgets::popovers::new_list::NewListModel;

pub struct StateModel {
    message: Option<StateMsg>,
}

impl Model for StateModel {
    type Msg = StateMsg;
    type Widgets = StateWidgets;
    type Components = StateComponents;
}

pub enum StateMsg {
    UpdateContent(ContentMsg),
    UpdateSidebar(SidebarMsg),
    Folded,
    Unfolded,
    Forward,
    Back
}

pub struct StateComponents {
    sidebar: RelmComponent<SidebarModel, StateModel>,
    content: RelmComponent<ContentModel, StateModel>,
    new_list: RelmComponent<NewListModel, StateModel>,
    theme_selector: RelmComponent<ThemeSelector, StateModel>,
}

impl Components<StateModel> for StateComponents {
    fn init_components(parent_model: &StateModel, parent_sender: Sender<StateMsg>) -> Self {
        Self {
            sidebar: RelmComponent::new(parent_model, parent_sender.clone()),
            content: RelmComponent::new(parent_model, parent_sender.clone()),
            new_list: RelmComponent::new(parent_model, parent_sender.clone()),
            theme_selector: RelmComponent::new(parent_model, parent_sender),
        }
    }

    fn connect_parent(&mut self, _parent_widgets: &StateWidgets) {}
}

impl ComponentUpdate<AppModel> for StateModel {
    fn init_model(_: &AppModel) -> Self {
        Self {
            message: None
        }
    }

    fn update(&mut self, msg: Self::Msg, components: &Self::Components, _sender: Sender<Self::Msg>, _parent_sender: Sender<AppMsg>) {
        match msg {
            StateMsg::UpdateContent(msg) => {
                components.content.send(msg).unwrap()
            }
            StateMsg::UpdateSidebar(msg) => {
                components.sidebar.send(msg).unwrap()
            }
            _ => self.message = Some(msg)
        }
    }
}

#[relm4_macros::widget(pub)]
impl Widgets<StateModel, AppModel> for StateWidgets {
    fn pre_view() {
        if let Some(msg) = &model.message {
            match msg {
                StateMsg::Folded => {
                    self.leaflet.set_visible_child(&self.content);
                    self.go_back_button.set_visible(true);
                    sidebar_header.set_show_start_title_buttons(true);
                    sidebar_header.set_show_end_title_buttons(true);
                }
                StateMsg::Unfolded => {
                    self.go_back_button.set_visible(false);
                    sidebar_header.set_show_start_title_buttons(false);
                    sidebar_header.set_show_end_title_buttons(false);
                }
                StateMsg::Forward => self.leaflet.set_visible_child(&self.content),
                StateMsg::Back => self.leaflet.set_visible_child(&self.sidebar),
                _ => {}
            }
        }
    }

    view! {
        leaflet = &adw::Leaflet {
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
                        set_popover = Some(&gtk::Popover) {
                             set_child = Some(&gtk::Box) {
                                set_orientation: gtk::Orientation::Vertical,
                                append: &components.theme_selector.widgets().unwrap().theme_selector,
                                append = &gtk::Button {
                                    set_label: "About"
                                }
                            },
                        },
                    },
                    pack_start: new_list_button = &gtk::MenuButton {
                        set_icon_name: "value-increase-symbolic",
                        add_css_class: "raised",
                        set_has_frame: true,
                        set_direction: gtk::ArrowType::None,
                        set_popover: Some(components.new_list.root_widget())
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
                            send!(sender, StateMsg::Back);
                        }
                    },
                },
                append: components.content.root_widget()
            },
            connect_folded_notify: clone!(@weak content, @strong sender => move |leaflet| {
                if leaflet.is_folded() {
                    leaflet.set_visible_child(&content);
                    send!(sender, StateMsg::Folded);
                } else {
                    send!(sender, StateMsg::Unfolded);
                }
            })
        }
    }
}