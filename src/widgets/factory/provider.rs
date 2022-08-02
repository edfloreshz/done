use crate::data::models::generic::lists::GenericList;
use crate::widgets::components::sidebar::SidebarInput;
use crate::{StaticProviderType, PLUGINS};
use adw::prelude::{ExpanderRowExt, PreferencesGroupExt, PreferencesRowExt};
use relm4::adw;
use relm4::factory::{
	DynamicIndex, FactoryComponent, FactoryComponentSender, FactoryVecDeque,
	FactoryView,
};
use relm4::gtk;
use relm4::gtk::prelude::WidgetExt;

#[allow(dead_code)]
#[derive(Debug)]
pub struct ProviderModel {
	pub provider: StaticProviderType,
	pub list_factory: FactoryVecDeque<GenericList>,
}

#[derive(Debug)]
pub enum ProviderInput {
	SelectSmartProvider,
	AddList(String, String),
	RemoveList(DynamicIndex),
	RenameList(DynamicIndex, String),
	ListSelected(GenericList),
}

#[derive(Debug)]
pub enum ProviderOutput {
	ListSelected(GenericList),
}

#[relm4::factory(pub)]
impl FactoryComponent for ProviderModel {
	type ParentMsg = SidebarInput;
	type ParentWidget = gtk::Box;
	type CommandOutput = ();
	type Input = ProviderInput;
	type Output = ProviderOutput;
	type InitParams = StaticProviderType;
	type Widgets = ProviderWidgets;

	view! {
		#[root]
		#[name(list_box)]
		adw::PreferencesGroup {
			#[name(expander)]
			add = &adw::ExpanderRow {
				set_title: self.provider.get_name(),
				set_subtitle: self.provider.get_description(),
				set_icon_name: Some(self.provider.get_icon_name()),
				set_enable_expansion: !self.provider.is_smart(),
				set_show_enable_switch: !self.provider.is_smart(),
				set_expanded: self.provider.is_smart(),
			},
			add_controller = &gtk::GestureClick {
				connect_pressed[sender] => move |_, _, _, _| {
					sender.input.send(ProviderInput::SelectSmartProvider)
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
			provider: params,
			list_factory: FactoryVecDeque::new(
				adw::ExpanderRow::default(),
				&sender.input,
			),
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
		if !self.provider.is_smart() {
			self.list_factory =
				FactoryVecDeque::new(widgets.expander.clone(), &sender.input);
			for list in self.provider.read_task_lists().unwrap() {
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
			ProviderInput::SelectSmartProvider => {
				let mut list = GenericList::new(
					self.provider.get_name(),
					self.provider.get_icon_name(),
					0,
					self.provider.get_id(),
				);
				if self.provider.is_smart() {
					list.make_smart();
				}
				sender.input.send(ProviderInput::ListSelected(list))
			},
			ProviderInput::RemoveList(_) => {},
			ProviderInput::AddList(provider, name) => {
				let current_provider = PLUGINS.get_provider(&provider);
				let new_list = current_provider
					.create_task_list(&provider, &name, "list-compact-symbolic")
					.expect("Failed to post task.");
				self.list_factory.guard().push_back(new_list)
			},
			ProviderInput::RenameList(_, _) => todo!(),
			ProviderInput::ListSelected(list) => {
				sender.output.send(ProviderOutput::ListSelected(list))
			},
		}
	}

	fn output_to_parent_msg(output: Self::Output) -> Option<Self::ParentMsg> {
		match output {
			ProviderOutput::ListSelected(list) => {
				Some(SidebarInput::ListSelected(list))
			},
		}
	}
}
