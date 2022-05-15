use glib::{clone, Sender};
use relm4::{ComponentUpdate, gtk, Model, send, Widgets};
use relm4::gtk::prelude::{
    BoxExt, ButtonExt, EditableExt, EntryBufferExtManual, EntryExt, OrientableExt, PopoverExt,
    WidgetExt,
};

use crate::widgets::app::{AppModel, AppMsg};
use crate::widgets::panel::sidebar::SidebarMsg;

pub struct NewListModel;

pub enum NewListMsg {
    AddList(String),
}

impl Model for NewListModel {
    type Msg = NewListMsg;
    type Widgets = NewListWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for NewListModel {
    fn init_model(_: &AppModel) -> Self {
        Self
    }

    fn update(
        &mut self,
        msg: Self::Msg,
        _components: &Self::Components,
        _sender: Sender<Self::Msg>,
        parent_sender: Sender<AppMsg>,
    ) {
        match msg {
            NewListMsg::AddList(title) => send!(
                parent_sender,
                AppMsg::UpdateSidebar(SidebarMsg::AddList(title))
            ),
        }
    }
}

#[relm4::widget(pub)]
impl Widgets<NewListModel, AppModel> for NewListWidgets {
    view! {
        new_list_popover = Some(&gtk::Popover) {
            set_child = Some(&gtk::Stack) {
                add_child = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,
                    append: &gtk::Label::new(Some("List Name")),
                    append = &gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 10,
                        append: new_list_entry = &gtk::Entry {
                            connect_activate(sender) => move |entry| {
                                let buffer = entry.buffer();
                                if !buffer.text().is_empty() {
                                    send!(sender, NewListMsg::AddList(buffer.text()))
                                }
                            }
                        },
                        append: providers_button = &gtk::MenuButton {
                            set_visible: false,
                            set_icon_name: "x-office-address-book-symbolic",
                            add_css_class: "raised",
                            set_has_frame: true,
                            set_direction: gtk::ArrowType::Right,
                            set_popover = Some(&gtk::Popover) {
                                set_child = Some(&gtk::Stack) {
                                    add_child = &gtk::Label {
                                        set_text: "Providers"
                                    }
                                }
                            }
                        }
                    },
                    append: add_button = &gtk::Button {
                        set_label: "Create List",
                        set_css_classes: &["suggested-action"],
                        connect_clicked: clone!(@weak new_list_entry, @strong sender => move |_| {
                            let buffer = new_list_entry.buffer();
                            if !buffer.text().is_empty() {
                                send!(sender, NewListMsg::AddList(buffer.text()))
                            }
                            new_list_entry.set_text("");
                        })
                    },
                    append: cancel_button = &gtk::Button {
                        set_label: "Cancel",
                        connect_clicked: clone!(@weak new_list_popover, @weak new_list_entry, @strong sender => move |_| {
                            new_list_entry.set_text("");
                            new_list_popover.popdown();
                        })
                    },
                }
            }
        }
    }
}
