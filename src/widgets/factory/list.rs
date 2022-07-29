use relm4::factory::{DynamicIndex, FactoryComponent, FactoryComponentSender, FactoryView};
use relm4::Sender;
use crate::core::models::generic::lists::GenericList;

use crate::gtk;
use crate::gtk::prelude::{OrientableExt, WidgetExt};
use crate::widgets::component::sidebar::SidebarInput;

#[derive(Debug)]
pub enum ListType {
	Inbox(i8),
	Today(i8),
	Next7Days(i8),
	All(i8),
	Starred(i8),
	Archived(i8),
	Other(usize, i8),
}

#[derive(Debug)]
pub enum ListInput {
	Rename(String),
	UpdateCount(i32),
	ChangeIcon(String),
}

#[derive(Debug)]
pub enum ListOutput {
	RemoveList(DynamicIndex),
}

#[relm4::factory(pub)]
impl FactoryComponent for GenericList {
	type ParentMsg = SidebarInput;
	type ParentWidget = gtk::ListBox;
	type CommandOutput = ();
	type Input = ListInput;
	type Output = ListOutput;
	type InitParams = GenericList;
	type Widgets = ListWidgets;

	view! {
		list_box = gtk::Box {
			set_orientation: gtk::Orientation::Horizontal,
			#[name = "icon"]
			gtk::Image {
				set_from_icon_name: Some(self.icon_name.as_ref().unwrap())
			},
			#[name = "name"]
			gtk::Label {
				set_halign: gtk::Align::Start,
				set_hexpand: true,
				set_text: self.display_name.as_str(),
				set_margin_top: 10,
				set_margin_bottom: 10,
				set_margin_start: 15,
				set_margin_end: 15,
			},
			#[name = "count"]
			gtk::Label {
				set_halign: gtk::Align::End,
				set_css_classes: &["dim-label", "caption"],
				#[watch]
				set_text: self.count.to_string().as_str(),
				set_margin_top: 10,
				set_margin_bottom: 10,
				set_margin_start: 15,
				set_margin_end: 15,
			}
		}
	}

	fn output_to_parent_msg(output: Self::Output) -> Option<SidebarInput> {
		Some(match output {
			ListOutput::RemoveList(index) => SidebarInput::RemoveList(index),
		})
	}

	fn init_model(params: Self::InitParams, index: &DynamicIndex, sender: &FactoryComponentSender<Self>) -> Self {
		params
	}

	fn init_widgets(&mut self, index: &DynamicIndex, root: &Self::Root, returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget, sender: &FactoryComponentSender<Self>) -> Self::Widgets {
		let widgets = view_output!();
		widgets
	}

	fn update(&mut self, message: Self::Input, sender: &FactoryComponentSender<Self>) {
		match message {
			ListInput::Rename(name) => self.display_name = name,
			ListInput::UpdateCount(count) => self.count = count,
			ListInput::ChangeIcon(icon) => {
				if icon.is_empty() {
					self.icon_name = None
				} else {
					self.icon_name = Some(icon)
				}
			},
		}
	}
}
