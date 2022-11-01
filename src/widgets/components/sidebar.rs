use relm4::adw;
use relm4::factory::FactoryVecDeque;
use relm4::{
	gtk,
	gtk::prelude::{BoxExt, OrientableExt, WidgetExt},
	ComponentParts, ComponentSender, SimpleComponent,
};

use crate::plugins::client::Plugin;
use crate::plugins::client::provider::List;
use crate::rt;
use crate::widgets::factory::provider::{ProviderInput, ProviderModel};

#[derive(Debug)]
pub struct SidebarModel {
	provider_factory: FactoryVecDeque<ProviderModel>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum SidebarInput {
	AddTaskList(usize, String, String),
	ListSelected(List),
	ProviderSelected(Plugin),
	RemoveService(String),
	Forward,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum SidebarOutput {
	ListSelected(List),
	ProviderSelected(Plugin),
	Forward,
}

#[relm4::component(pub)]
impl SimpleComponent for SidebarModel {
	type Input = SidebarInput;
	type Output = SidebarOutput;
	type Init = Option<SidebarModel>;
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
					}
				}
			},
		}
	}

	fn init(
		_init: Self::Init,
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
		
		for provider in Plugin::list() {
			if let Ok(connector) = rt().block_on(provider.connect()) {
				model.provider_factory.guard().push_back((provider, connector));
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
			SidebarInput::Forward => sender.output.send(SidebarOutput::Forward),
			SidebarInput::ProviderSelected(provider) => sender.output.send(SidebarOutput::ProviderSelected(provider))
		}
	}
}
