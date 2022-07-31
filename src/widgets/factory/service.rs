use adw::prelude::ExpanderRowExt;
use adw::prelude::PreferencesGroupExt;
use adw::prelude::PreferencesRowExt;
use relm4::adw;
use relm4::factory::{DynamicIndex, FactoryComponent, FactoryComponentSender, FactoryVecDeque, FactoryView};
use relm4::gtk;
use crate::data::models::generic::lists::GenericList;
use crate::data::models::generic::tasks::GenericTask;
use crate::data::plugins::local::LocalProvider;

use crate::data::traits::provider::Provider;
use crate::widgets::component::sidebar::SidebarInput;

#[allow(dead_code)]
#[derive(Debug)]
pub struct ServiceModel {
	pub provider: LocalProvider,
	pub lists: Option<FactoryVecDeque<GenericList>>,
	pub tasks: Option<FactoryVecDeque<GenericTask>>
}

#[derive(Debug)]
pub enum ServiceInput {
	UpdateService,
	RemoveList(DynamicIndex),
}

#[derive(Debug)]
pub enum ServiceOutput {}

#[relm4::factory(pub)]
impl FactoryComponent for ServiceModel {
	type ParentMsg = SidebarInput;
	type ParentWidget = gtk::Box;
	type CommandOutput = ();
	type Input = ServiceInput;
	type Output = ServiceOutput;
	type InitParams = ServiceModel;
	type Widgets = ProviderWidgets;

	view! {
			#[name = "list_box"]
			adw::PreferencesGroup {
					#[name = "expander"]
					add = &adw::ExpanderRow {
							set_title: self.provider.get_name(),
							add_prefix: &self.provider.get_icon(),
							add_action: &gtk::Button::from_icon_name("accessories-text-editor-symbolic")
					}
			}
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
		_sender: FactoryComponentSender<Self>,
	) {
		match message {
			ServiceInput::UpdateService => {
				todo!("Update lists")
			}
			ServiceInput::RemoveList(_) => {}
		}
	}

	fn init_widgets(
		&mut self,
		_index: &DynamicIndex,
		root: &Self::Root,
		_returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
		sender: FactoryComponentSender<Self>,
	) -> Self::Widgets {
		let widgets = view_output!();
		let fac = FactoryVecDeque::new(
			widgets.expander.clone(),
			&sender.input,
		);
		self.lists = Some(fac);
		//TODO: Iter list of task lists and create rows for each.
		// for list in self.lists.unwrap() {
		// 	relm4::view! {
		// 			#[name = "nested"]
		// 			&adw::ActionRow {
		// 					set_title: &list.display_name
		// 			}
		// 	}
		// 	widgets.expander.add_row(&nested)
		// }
		widgets
	}
}
