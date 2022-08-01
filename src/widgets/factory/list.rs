use relm4::factory::{
	DynamicIndex, FactoryComponent, FactoryComponentSender, FactoryView,
};

use crate::data::models::generic::lists::GenericList;
use crate::gtk::prelude::{WidgetExt, ButtonExt, ListBoxRowExt};
use crate::widgets::factory::provider::ServiceInput;
use crate::{adw, gtk};
use relm4::adw::prelude::{ActionRowExt, PreferencesRowExt};

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
	Select(usize),
	Rename(String),
	UpdateCount(i32),
	ChangeIcon(String),
}

#[derive(Debug)]
pub enum ListOutput {
	Select(GenericList)
}

#[relm4::factory(pub)]
impl FactoryComponent for GenericList {
	type ParentMsg = ServiceInput;
	type ParentWidget = gtk::ListBox;
	type CommandOutput = ();
	type Input = ListInput;
	type Output = ListOutput;
	type InitParams = GenericList;
	type Widgets = ListWidgets;

	view! {
		#[root]
		gtk::ListBoxRow {
			#[wrap(Some)]
			set_child = &adw::ActionRow {
				add_prefix = &gtk::Button {
					set_icon_name: self.icon_name.as_ref().unwrap(),
					set_css_classes: &["flat", "image-button"],
					set_valign: gtk::Align::Center
				},
				set_title: &self.display_name,
				add_suffix = &gtk::Label {
					set_halign: gtk::Align::End,
					set_css_classes: &["dim-label", "caption"],
					#[watch]
					set_text: self.count.to_string().as_str(),
					set_margin_top: 10,
					set_margin_bottom: 10,
					set_margin_start: 15,
					set_margin_end: 15,
				},
				add_suffix = &gtk::Button {
					set_icon_name: "user-trash-full-symbolic",
					set_css_classes: &["circular", "image-button", "destructive-action"],
					set_valign: gtk::Align::Center
				},
			}
		}
	}

	fn init_widgets(
		&mut self,
		index: &DynamicIndex,
		root: &Self::Root,
		_returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
		sender: FactoryComponentSender<Self>,
	) -> Self::Widgets {
		let widgets = view_output!();
		widgets
	}

	fn init_model(
		params: Self::InitParams,
		_index: &DynamicIndex,
		_sender: FactoryComponentSender<Self>,
	) -> Self {
		params
	}

	fn update(
		&mut self,
		message: Self::Input,
		sender: FactoryComponentSender<Self>,
	) {
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
			ListInput::Select(index) => {
				sender.output.send(ListOutput::Select(self.clone()))
			}
		}
	}

	fn output_to_parent_msg(output: Self::Output) -> Option<Self::ParentMsg> {
		match output { ListOutput::Select(list) => Some(ServiceInput::ListSelected(list)) }
	}
}
