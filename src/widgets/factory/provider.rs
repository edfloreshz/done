use crate::data::models::generic::lists::GenericList;
use crate::widgets::components::sidebar::SidebarInput;
use adw::prelude::{PreferencesRowExt, ExpanderRowExt, PreferencesGroupExt};
use relm4::adw;
use relm4::gtk::prelude::WidgetExt;
use relm4::factory::{
	DynamicIndex, FactoryComponent, FactoryComponentSender, FactoryVecDeque,
	FactoryView,
};
use relm4::gtk;
use crate::{ProviderType, SERVICES};

#[allow(dead_code)]
#[derive(Debug)]
pub struct ServiceModel {
	pub service: ProviderType,
	pub list_factory: FactoryVecDeque<GenericList>,
}

#[derive(Debug)]
pub enum ServiceInput {
	SelectSmartProvider,
	AddList(String, String),
	RemoveList(DynamicIndex),
	RenameList(DynamicIndex, String),
	ListSelected(GenericList),
}

#[derive(Debug)]
pub enum ServiceOutput {
	ListSelected(GenericList),
}

#[relm4::factory(pub)]
impl FactoryComponent for ServiceModel {
	type ParentMsg = SidebarInput;
	type ParentWidget = gtk::Box;
	type CommandOutput = ();
	type Input = ServiceInput;
	type Output = ServiceOutput;
	type InitParams = ProviderType;
	type Widgets = ProviderWidgets;

	view! {
		#[root]
		#[name(list_box)]
		adw::PreferencesGroup {
			#[name(expander)]
			add = &adw::ExpanderRow {
				set_title: self.service.get_name(),
				set_subtitle: self.service.get_description(),
				set_icon_name: Some(self.service.get_icon_name()),
				set_enable_expansion: !self.service.is_smart(),
				set_show_enable_switch: !self.service.is_smart(),
				set_expanded: self.service.is_smart(),
			},
			add_controller = &gtk::GestureClick {
				connect_pressed[sender] => move |_, _, _, _| {
					sender.input.send(ServiceInput::SelectSmartProvider)
				}
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
			list_factory: FactoryVecDeque::new(adw::ExpanderRow::default(), &sender.input),
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
		if !self.service.is_smart() {
			self.list_factory = FactoryVecDeque::new(widgets.expander.clone(), &sender.input);
			for list in self.service.read_task_lists().unwrap() {
				self.list_factory.guard().push_back(list)
			}
		}
		widgets
	}

	fn update(
		&mut self,
		message: Self::Input,
		sender: FactoryComponentSender<Self>,
	) {
		match message {
			ServiceInput::SelectSmartProvider => {
				if self.service.is_smart() {
					let mut list = GenericList::new(
						self.service.get_name(),
						self.service.get_icon_name(),
						0,
						self.service.get_id()
					);
					list.make_smart();
					sender.input.send(ServiceInput::ListSelected(list));
				}
			},
			ServiceInput::RemoveList(_) => {},
			ServiceInput::AddList(provider, name) => {
				let services = unsafe { &*SERVICES.get_mut().unwrap() };
				let service = services.iter().find(|l| l.get_id() == provider);
				if let Some(service) = service {
					let new_list = service.create_task_list(&provider, &name, "list-compact-symbolic").expect("Failed to post task.");
					self
						.list_factory
						.guard()
						.push_back(new_list)
				}
			},
			ServiceInput::RenameList(_, _) => todo!(),
			ServiceInput::ListSelected(list) => {
				sender.output.send(ServiceOutput::ListSelected(list))
			},
		}
	}

	fn output_to_parent_msg(output: Self::Output) -> Option<Self::ParentMsg> {
		match output { ServiceOutput::ListSelected(list) => Some(SidebarInput::ListSelected(list)) }
	}
}
