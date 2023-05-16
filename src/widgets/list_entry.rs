use glib::clone;
use gtk::prelude::{
	BoxExt, ButtonExt, EditableExt, EntryBufferExtManual, EntryExt, WidgetExt,
};
use relm4::{
	adw,
	gtk::{
		self,
		traits::{GtkWindowExt, OrientableExt},
	},
	ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent,
};

use crate::fl;

#[derive(Debug)]
pub struct ListEntryComponent;

#[derive(Debug)]
pub enum ListEntryOutput {
	AddTaskListToSidebar(String),
}

#[relm4::component(pub)]
impl SimpleComponent for ListEntryComponent {
	type Input = ();
	type Output = ListEntryOutput;
	type Init = ();

	view! {
		#[root]
		adw::Window {
			set_hide_on_close: true,
			set_default_width: 320,
			set_resizable: false,
			set_modal: true,

			gtk::Box {
				set_orientation: gtk::Orientation::Vertical,

				adw::HeaderBar {
					set_show_end_title_buttons: true,
					set_css_classes: &["flat"],
					set_title_widget: Some(&gtk::Box::default())
				},
				gtk::Box {
					set_orientation: gtk::Orientation::Vertical,
					set_margin_all: 20,
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
						set_label: fl!("add-list"),
						connect_clicked: clone!(@strong new_list_entry, @strong sender => move |_| {
							let buffer = new_list_entry.buffer();
							if !buffer.text().is_empty() {
								sender.output(ListEntryOutput::AddTaskListToSidebar(buffer.text().to_string())).unwrap_or_default();
							}
							new_list_entry.set_text("");
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
			model: ListEntryComponent,
			widgets,
		}
	}
}
