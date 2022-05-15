use glib::clone;
use gtk::prelude::{
    BoxExt, ButtonExt, EditableExt, EntryBufferExtManual, EntryExt, OrientableExt,
    PopoverExt, WidgetExt,
};
use relm4::{ComponentParts, ComponentSender, gtk, SimpleComponent, view};

pub struct NewListModel;

pub enum NewListOutput {
    AddNewList(String)
}

#[relm4::component(pub)]
impl SimpleComponent for NewListModel {
    type Input = ();
    type Output = NewListOutput;
    type InitParams = ();
    type Widgets = NewListWidgets;

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
                                    sender.output.send(NewListOutput::AddNewList(buffer.text()))
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
                        connect_clicked: clone!(@strong new_list_entry, @strong sender => move |_| {
                            let buffer = new_list_entry.buffer();
                            if !buffer.text().is_empty() {
                                sender.output.send(NewListOutput::AddNewList(buffer.text()))
                            }
                            new_list_entry.set_text("");
                        })
                    },
                    append: cancel_button = &gtk::Button {
                        set_label: "Cancel",
                        connect_clicked: clone!(@strong new_list_popover, @strong new_list_entry, @strong sender => move |_| {
                            new_list_entry.set_text("");
                            new_list_popover.popdown();
                        })
                    },
                }
            }
        }
    }

    fn init(params: Self::InitParams, root: &Self::Root, sender: &ComponentSender<Self>) -> ComponentParts<Self> {
        let widgets = view_output!();
        let model = NewListModel;
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: &ComponentSender<Self>) {}
}
