use relm4::adw;
use relm4::factory::FactoryVecDeque;
use relm4::{
	gtk,
	gtk::prelude::{BoxExt, OrientableExt, WidgetExt},
	ComponentParts, ComponentSender, SimpleComponent,
};

use crate::data::models::generic::lists::GenericList;
use crate::widgets::popover::providers_list::{ServiceInput, ServiceModel};
use crate::SERVICES;

#[derive(Debug)]
pub struct SidebarModel {
	service_factory: FactoryVecDeque<ServiceModel>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum SidebarInput {
	AddTaskList(String, String),
	RemoveService(String),
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum SidebarOutput {
	ListSelected(usize, String, GenericList),
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
			append: scroll_window = &gtk::ScrolledWindow {
				#[wrap(Some)]
				set_child: clamp = &adw::Clamp {
					#[wrap(Some)]
						set_child: providers_container = &gtk::Box {
							set_margin_top: 5,
							set_margin_start: 10,
							set_margin_end: 10,
							set_orientation: gtk::Orientation::Vertical,
							set_spacing: 12,
							set_vexpand: true,
							set_css_classes: &["navigation-sidebar"],
							// connect_row_activated[sender] => move |listbox, _| {
							// 	let index = listbox.selected_row().unwrap().index() as usize;
							// 	sender.input(SidebarInput::ListSelected(index));
							// 	sender.output(SidebarOutput::Forward)
							// },
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
					model.service_factory.guard().push_back(service);
				}
			}
		}
		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		match message {
			SidebarInput::AddTaskList(provider, name) => {
				self
					.service_factory
					.send(0, ServiceInput::AddList(provider, name));
			},
			SidebarInput::RemoveService(_) => todo!(),
		}
	}
}
