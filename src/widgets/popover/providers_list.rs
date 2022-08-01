use gtk::prelude::BoxExt;
use gtk::prelude::ListBoxRowExt;
use relm4::factory::{
	DynamicIndex, FactoryComponent, FactoryComponentSender, FactoryView,
};
use relm4::gtk;

use crate::ProviderType;
use crate::widgets::popover::new_list::NewListOutput;

#[derive(Debug)]
pub struct ProvidersList {
	pub(crate) provider: ProviderType
}

#[derive(Debug)]
pub enum ProviderInput {}

#[relm4::factory(pub)]
impl FactoryComponent for ProvidersList {
	type ParentMsg = NewListOutput;
	type ParentWidget = gtk::ListBox;
	type CommandOutput = ();
	type Input = ProviderInput;
	type Output = ();
	type InitParams = ProviderType;
	type Widgets = ProviderWidgets;

	view! {
			#[name = "providers_container"]
			gtk::ListBoxRow {
				set_activatable: true,
				#[wrap(Some)]
				set_child = &gtk::Box {
					append = &gtk::Image {
						set_icon_name: Some(&self.provider.get_icon_name())
					},
					append = &gtk::Label {
						set_label: self.provider.get_name()
					}
				}
		}
	}

	fn init_model(
		params: Self::InitParams,
		_index: &DynamicIndex,
		_sender: FactoryComponentSender<Self>,
	) -> Self {
		Self {
			provider: params
		}
	}

	fn update(
		&mut self,
		_message: Self::Input,
		_sender: FactoryComponentSender<Self>,
	) {
	}

	fn init_widgets(
		&mut self,
		_index: &DynamicIndex,
		root: &Self::Root,
		_returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
		_sender: FactoryComponentSender<Self>,
	) -> Self::Widgets {
		let widgets = view_output!();
		widgets
	}
}
