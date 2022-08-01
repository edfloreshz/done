use relm4::adw;
use relm4::factory::FactoryVecDeque;
use relm4::{
	gtk,
	gtk::prelude::{BoxExt, OrientableExt, WidgetExt},
	ComponentParts, ComponentSender, SimpleComponent,
};

use crate::data::models::generic::lists::GenericList;
use crate::widgets::factory::provider::{ServiceInput, ServiceModel};
use crate::SERVICES;

#[derive(Debug)]
pub struct SidebarModel {
	service_factory: FactoryVecDeque<ServiceModel>,
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
			service_factory: FactoryVecDeque::new(
				widgets.providers_container.clone(),
				&sender.input,
			),
		};
		unsafe {
			for service in &mut *SERVICES.get_mut().unwrap() {
				if service.is_enabled() {
					model.service_factory.guard().push_back(&**service);
				}
			}
		}
		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
		match message {
			SidebarInput::AddTaskList(index, provider, name) => {
				self
					.service_factory
					.send(index, ServiceInput::AddList(provider, name));
			},
			SidebarInput::RemoveService(_) => todo!(),
			SidebarInput::ListSelected(list) => sender.output.send(SidebarOutput::ListSelected(list))
		}
	}
}
