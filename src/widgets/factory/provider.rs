use crate::data::models::generic::lists::GenericTaskList;
use crate::widgets::components::sidebar::SidebarInput;
use crate::{StaticProviderType, PLUGINS};
use adw::prelude::{ExpanderRowExt, PreferencesGroupExt, PreferencesRowExt};
use relm4::{adw, Component, Controller};
use relm4::factory::{
	DynamicIndex, FactoryComponent, FactoryComponentSender, FactoryVecDeque,
	FactoryView,
};
use relm4::gtk;
use relm4::gtk::prelude::WidgetExt;
use relm4::ComponentController;
use crate::widgets::popover::new_list::{NewListModel, NewListOutput};

#[allow(dead_code)]
#[derive(Debug)]
pub struct ProviderModel {
	pub provider: StaticProviderType,
	pub list_factory: FactoryVecDeque<GenericTaskList>,
	pub new_list_controller: Controller<NewListModel>,
}

#[derive(Debug)]
pub enum ProviderInput {
	SelectSmartProvider,
	AddList(String, String),
	DeleteTaskList(DynamicIndex),
	RenameList(DynamicIndex, String),
	ListSelected(GenericTaskList),
	Forward(bool)
}

#[derive(Debug)]
pub enum ProviderOutput {
	ListSelected(GenericTaskList),
	Forward
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
				set_expanded: self.provider.is_smart(),
			},
			add_controller = &gtk::GestureClick {
				connect_pressed[sender, index] => move |_, _, _, _| {
					sender.input.send(ProviderInput::SelectSmartProvider);
					sender.input.send(ProviderInput::Forward(index.clone().current_index() <= 3))
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
			new_list_controller: NewListModel::builder()
				.launch(())
				.forward(&sender.input, |message| match message {
					NewListOutput::AddTaskListToSidebar(name) => {
						ProviderInput::AddList(params.get_id().into(), name)
					},
				})
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
		if !self.provider.is_smart() {
			self.list_factory =
				FactoryVecDeque::new(widgets.expander.clone(), &sender.input);
			for list in self.provider.read_task_lists().unwrap() {
				self.list_factory.guard().push_back(list)
			}
			relm4::view! {
				#[name(new_list_button)]
				gtk::MenuButton {
					set_icon_name: "value-increase-symbolic",
					set_css_classes: &["flat", "image-button"],
					set_valign: gtk::Align::Center,
					set_popover: Some(self.new_list_controller.widget())
				}
			}
			widgets.expander.add_action(&new_list_button);
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
				let mut list = GenericTaskList::new(
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
			ProviderInput::DeleteTaskList(index) => {
				self.list_factory.guard().remove(index.current_index());
			},
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
			ProviderInput::Forward(forward) => if forward {
				sender.output.send(ProviderOutput::Forward)
			}
		}
	}

	fn output_to_parent_msg(output: Self::Output) -> Option<Self::ParentMsg> {
		match output {
			ProviderOutput::ListSelected(list) => {
				Some(SidebarInput::ListSelected(list))
			},
			ProviderOutput::Forward => Some(SidebarInput::Forward)
		}
	}
}
