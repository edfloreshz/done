use relm4::adw;
use relm4::factory::FactoryVecDeque;
use relm4::{
	gtk,
	gtk::prelude::{BoxExt, OrientableExt, WidgetExt},
	ComponentParts, ComponentSender, SimpleComponent,
};

use crate::data::models::generic::lists::GenericList;
use crate::widgets::factory::provider::{ProviderInput, ProviderModel};
use crate::PLUGINS;

#[derive(Debug)]
pub struct SidebarModel {
	provider_factory: FactoryVecDeque<ProviderModel>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum SidebarInput {
	AddTaskList(usize, String, String),
	ListSelected(GenericList),
	RemoveService(String),
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum SidebarOutput {
	ListSelected(GenericList),
	Forward,
}

#[relm4::component(pub)]
impl SimpleComponent for SidebarModel {
	type Input = SidebarInput;
	type Output = SidebarOutput;
	type InitParams = Option<SidebarModel>;
	type Widgets = SidebarWidgets;

	view! {
		sidebar = &gtk::Box {
			set_orientation: gtk::Orientation::Vertical,
			#[name(scroll_window)]
			gtk::ScrolledWindow {
				#[name(clamp)]
				adw::Clamp {
					#[name(providers_container)]
					gtk::Box {
						set_margin_top: 5,
						set_margin_start: 10,
						set_margin_end: 10,
						set_orientation: gtk::Orientation::Vertical,
						set_spacing: 12,
						set_vexpand: true,
						set_css_classes: &["navigation-sidebar"],
					},
				}
			},
		}
	}

	fn init(
		_params: Self::InitParams,
		root: &Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let widgets = view_output!();
		let mut model = SidebarModel {
			provider_factory: FactoryVecDeque::new(
				widgets.providers_container.clone(),
				&sender.input,
			),
		};
		for provider in PLUGINS.get_providers() {
			if provider.is_enabled() {
				model.provider_factory.guard().push_back(&*provider);
			}
		}
		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
		match message {
			SidebarInput::AddTaskList(index, provider, name) => {
				self
					.provider_factory
					.send(index, ProviderInput::AddList(provider, name));
			},
			SidebarInput::RemoveService(_) => todo!(),
			SidebarInput::ListSelected(list) => {
				sender.output.send(SidebarOutput::ListSelected(list))
			},
		}
	}
}
