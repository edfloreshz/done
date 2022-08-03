use relm4::factory::{
	DynamicIndex, FactoryComponent, FactoryComponentSender, FactoryView,
};

use crate::data::models::generic::lists::GenericTaskList;
use crate::gtk::prelude::{ButtonExt, WidgetExt, EditableExt};
use crate::widgets::factory::provider::ProviderInput;
use crate::{adw, gtk, PLUGINS};
use relm4::adw::prelude::ActionRowExt;


#[derive(Debug)]
pub enum ListInput {
	Select,
	Delete(DynamicIndex),
	Rename(String),
	ChangeIcon(String),
}

#[derive(Debug)]
pub enum ListOutput {
	Select(GenericTaskList),
	DeleteTaskList(DynamicIndex),
	Forward
}

#[relm4::factory(pub)]
impl FactoryComponent for GenericTaskList {
	type ParentMsg = ProviderInput;
	type ParentWidget = adw::ExpanderRow;
	type CommandOutput = ();
	type Input = ListInput;
	type Output = ListOutput;
	type InitParams = GenericTaskList;
	type Widgets = ListWidgets;

	view! {
		#[root]
		gtk::ListBoxRow {
			adw::ActionRow {
				add_prefix = &gtk::Entry {
					set_hexpand: false,
					add_css_class: "flat",
					add_css_class: "no-border",
					#[watch]
					set_text: self.display_name.as_str(),
					set_margin_top: 10,
					set_margin_bottom: 10
				},
				add_prefix = &gtk::Button {
					set_icon_name: self.icon_name.as_ref().unwrap(),
					set_css_classes: &["flat", "image-button"],
					set_valign: gtk::Align::Center
				},
				add_suffix = &gtk::Label {
					set_halign: gtk::Align::End,
					set_css_classes: &["dim-label", "caption"],
					#[watch]
					set_text: self.count.to_string().as_str(),
				},
				add_suffix = &gtk::Button {
					set_icon_name: "user-trash-full-symbolic",
					set_css_classes: &["circular", "image-button", "destructive-action"],
					set_valign: gtk::Align::Center,
					connect_clicked[sender, index] => move |_| {
						sender.input.send(ListInput::Delete(index.clone()))
					}
				},
			},
			add_controller = &gtk::GestureClick {
				connect_pressed[sender] => move |_, _, _, _| {
					sender.input.send(ListInput::Select);
					sender.output(ListOutput::Forward)
				}
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
		let service = PLUGINS.get_provider(&self.provider);
		match message {
			ListInput::Rename(name) => {
				let mut list = self.clone();
				list.display_name = name.clone();
				if service.update_task_list(list).is_ok() {
					self.display_name = name;
				}
			},
			ListInput::Delete(index) => {
				if service.remove_task_list(self.clone()).is_ok() {
					sender.output.send(ListOutput::DeleteTaskList(index))
				}
			},
			ListInput::ChangeIcon(icon) => {
				let mut list = self.clone();
				list.icon_name = Some(icon.clone());
				if service.update_task_list(list).is_ok() {
					self.icon_name = Some(icon);
				}
			},
			ListInput::Select => sender.output.send(ListOutput::Select(self.clone())),
		}
	}

	fn output_to_parent_msg(output: Self::Output) -> Option<Self::ParentMsg> {
		match output {
			ListOutput::Select(list) => Some(ProviderInput::ListSelected(list)),
			ListOutput::DeleteTaskList(index) => Some(ProviderInput::DeleteTaskList(index)),
			ListOutput::Forward => Some(ProviderInput::Forward(true))
		}
	}
}
