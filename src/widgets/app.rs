use once_cell::sync::OnceCell;
use relm4::{
    adw,
    adw::prelude::AdwApplicationWindowExt,
    gtk,
    gtk::prelude::{BoxExt, GtkWindowExt, OrientableExt, WidgetExt},
    AppUpdate, Components, Model, RelmComponent, Sender, Widgets, WidgetPlus,
};
use tokio::runtime::Runtime;
use tracker::track;

use crate::widgets::content::{ContentModel, ContentMsg};
use crate::widgets::details::DetailsModel;
use crate::widgets::sidebar::SidebarModel;

static RT: OnceCell<Runtime> = OnceCell::new();

#[track]
pub struct AppModel {
    #[tracker::no_eq]
    pub(crate) selected_list: String,
}

impl AppModel {
    pub fn new(selected_list: &str) -> Self {
        Self {
            selected_list: selected_list.to_string(),
            tracker: 0,
        }
    }
}

pub enum AppMsg {
    Login,
    ListSelected(String),
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
        self.reset();
        match msg {
            AppMsg::Login => {
                println!("Login...")
            }
            AppMsg::ListSelected(list_id) => {
                self.set_selected_list(list_id);
                components.content.send(ContentMsg::ParentUpdate(self.selected_list.clone())).unwrap();
            }
        }
        true
    }
}

pub struct AppComponents {
    sidebar: RelmComponent<SidebarModel, AppModel>,
    content: RelmComponent<ContentModel, AppModel>,
    details: RelmComponent<DetailsModel, AppModel>,
}

impl Components<AppModel> for AppComponents {
    fn init_components(parent_model: &AppModel, parent_sender: Sender<AppMsg>) -> Self {
        AppComponents {
            sidebar: RelmComponent::new(parent_model, parent_sender.clone()),
            content: RelmComponent::new(parent_model, parent_sender.clone()),
            details: RelmComponent::new(parent_model, parent_sender),
        }
    }

    fn connect_parent(&mut self, _parent_widgets: &AppWidgets) {}
}

#[relm4_macros::widget(pub)]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        window = adw::ApplicationWindow {
            set_default_width: 800,
            set_default_height: 700,
            set_width_request: 460,
            set_height_request: 700,

            set_content: overlay = Some(&gtk::Overlay) {
                set_child: stack = Some(&gtk::Stack) {
                    set_hexpand: true,
                    set_vexpand: true,
                    set_transition_duration: 250,
                    set_transition_type: gtk::StackTransitionType::Crossfade,
                    add_child: leaflet = &adw::Leaflet {
                        set_can_navigate_back: true,
                        append = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_width_request: 320,
                            append: list_header = &adw::HeaderBar {
                                set_show_end_title_buttons: false,
                                set_title_widget = Some(&gtk::Label) {
                                    set_label: "To Do",
                                },
                            },
                            append: &components.sidebar.widgets().unwrap().list_container
                        },
                        append: &gtk::Separator::default(),
                        append: content_box = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_hexpand: true,
                            set_vexpand: true,
                            append = &adw::HeaderBar {
                                set_hexpand: true,
                                set_show_end_title_buttons: true,
                            },
                            append: &components.content.widgets().unwrap().task_container,
                        }
                    },
                }
            },
        }
    }
}
