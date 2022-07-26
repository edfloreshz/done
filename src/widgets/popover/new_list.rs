use glib::clone;
use gtk::prelude::{
	BoxExt, ButtonExt, EditableExt, EntryBufferExtManual, EntryExt,
	OrientableExt, PopoverExt, WidgetExt,
};
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};
use crate::app::i18n::i18n;

pub struct NewListModel;

pub enum NewListOutput {
	AddNewList(String),
}

#[relm4::component(pub)]
impl SimpleComponent for NewListModel {
	type Input = ();
	type Output = NewListOutput;
	type InitParams = ();
	type Widgets = NewListWidgets;

	view! {
		#[root]
		gtk::Popover {
			#[wrap(Some)]
			set_child = &gtk::Stack {
				add_child = &gtk::Box {
					set_orientation: gtk::Orientation::Vertical,
					set_spacing: 10,
					gtk::Label::new(Some(&i18n("List Name"))),
					gtk::Box {
						set_orientation: gtk::Orientation::Horizontal,
						set_spacing: 10,
						#[name = "new_list_entry"]
						gtk::Entry {
							connect_activate[sender] => move |entry| {
								let buffer = entry.buffer();
								if !buffer.text().is_empty() {
									sender.output(NewListOutput::AddNewList(buffer.text()))
								}
							}
						},
						#[name = "providers_button"]
						gtk::MenuButton {
							set_visible: false,
							set_icon_name: "x-office-address-book-symbolic",
							add_css_class: "raised",
							set_has_frame: true,
							set_direction: gtk::ArrowType::Right,
							#[wrap(Some)]
							set_popover = &gtk::Popover {
								#[wrap(Some)]
								set_child = &gtk::Stack {
									add_child = &gtk::Label {
										set_text: &i18n("Providers")
									}
								}
							}
						}
					},
					#[name = "add_button"]
					gtk::Button {
						set_label: &i18n("Create List"),
						set_css_classes: &["suggested-action"],
						connect_clicked: clone!(@strong new_list_entry, @strong sender => move |_| {
							let buffer = new_list_entry.buffer();
							if !buffer.text().is_empty() {
								sender.output(NewListOutput::AddNewList(buffer.text()))
							}
							new_list_entry.set_text("");
						})
					},
					#[name = "cancel_button"]
					gtk::Button {
						set_label: &i18n("Cancel"),
						connect_clicked: clone!(@strong root, @strong new_list_entry, @strong sender => move |_| {
							new_list_entry.set_text("");
							root.popdown();
						})
					},
				}
			}
		}
	}

	fn init(
		_params: Self::InitParams,
		root: &Self::Root,
		sender: &ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let widgets = view_output!();
		let model = NewListModel;
		ComponentParts { model, widgets }
	}

	fn update(&mut self, _message: Self::Input, _sender: &ComponentSender<Self>) {
	}
}
