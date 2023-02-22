use glib::clone;
use gtk::prelude::{
	BoxExt, ButtonExt, EditableExt, EntryBufferExtManual, EntryExt,
	OrientableExt, PopoverExt, WidgetExt,
};
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

use crate::fl;

#[derive(Debug)]
pub struct ListEntryModel;

#[derive(Debug)]
pub enum ListEntryOutput {
	AddTaskListToSidebar(String),
}

#[relm4::component(pub)]
impl SimpleComponent for ListEntryModel {
	type Input = ();
	type Output = ListEntryOutput;
	type Init = ();

	view! {
		#[root]
		gtk::Popover {
			#[wrap(Some)]
			set_child = &gtk::Stack {
				add_child = &gtk::Box {
					set_orientation: gtk::Orientation::Horizontal,
					set_spacing: 10,
					#[name = "new_list_entry"]
					gtk::Entry {
						set_placeholder_text: Some(fl!("list-name")),
						connect_activate[sender] => move |entry| {
							let buffer = entry.buffer();
							if !buffer.text().is_empty() {
								sender.output(ListEntryOutput::AddTaskListToSidebar(buffer.text().to_string())).unwrap_or_default();
							}
						}
					},
					#[name = "add_button"]
					gtk::Button {
						set_icon_name: "checkmark-small-symbolic",
						set_css_classes: &["suggested-action"],
						connect_clicked: clone!(@strong new_list_entry, @strong sender => move |_| {
							let buffer = new_list_entry.buffer();
							if !buffer.text().is_empty() {
								sender.output(ListEntryOutput::AddTaskListToSidebar(buffer.text().to_string())).unwrap_or_default();
							}
							new_list_entry.set_text("");
						})
					},
					#[name = "cancel_button"]
					gtk::Button {
						set_icon_name: "small-x-symbolic",
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
		_init: Self::Init,
		root: &Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let widgets = view_output!();
		ComponentParts {
			model: ListEntryModel,
			widgets,
		}
	}
}
