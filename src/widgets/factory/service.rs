use crate::data::models::generic::lists::GenericList;
use crate::data::traits::provider::Provider;
use crate::widgets::component::sidebar::SidebarInput;
use adw::prelude::ExpanderRowExt;
use adw::prelude::PreferencesGroupExt;
use adw::prelude::PreferencesRowExt;
use relm4::adw;
use relm4::factory::{
	DynamicIndex, FactoryComponent, FactoryComponentSender, FactoryVecDeque,
	FactoryView,
};
use relm4::gtk;

#[allow(dead_code)]
#[derive(Debug)]
pub struct ServiceModel {
	pub service: &'static Box<dyn Provider + Sync>,
	pub lists: FactoryVecDeque<GenericList>,
}

#[derive(Debug)]
pub enum ServiceInput {
	UpdateService,
	AddList(String, String),
	RemoveList(DynamicIndex),
	RenameList(DynamicIndex, String),
	ListSelected(usize),
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
	type InitParams = &'static Box<dyn Provider + Sync>;
	type Widgets = ProviderWidgets;

	view! {
		#[name = "list_box"]
		adw::PreferencesGroup {
			#[name = "expander"]
			add = &adw::ExpanderRow {
				set_title: self.service.get_name(),
				add_prefix: &self.service.get_icon(),
				add_action: &gtk::Button::from_icon_name("accessories-text-editor-symbolic")
			}
		}
	}

	fn init_model(
		params: Self::InitParams,
		_index: &DynamicIndex,
		sender: FactoryComponentSender<Self>,
	) -> Self {
		Self {
			service: params,
			lists: FactoryVecDeque::new(adw::ExpanderRow::default(), &sender.input),
		}
	}

	fn update(
		&mut self,
		message: Self::Input,
		_sender: FactoryComponentSender<Self>,
	) {
		match message {
			ServiceInput::UpdateService => {
				todo!("Update lists")
			},
			ServiceInput::RemoveList(_) => {},
			ServiceInput::AddList(provider, name) => self
				.lists
				.guard()
				.push_back(GenericList::new(&name, "icon", 0, &provider)),
			ServiceInput::RenameList(_, _) => todo!(),
			ServiceInput::ListSelected(_) => todo!(),
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
		self.lists = FactoryVecDeque::new(widgets.expander.clone(), &sender.input);
		for list in self.service.read_task_lists().unwrap() {
			self.lists.guard().push_back(list)
		}
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
