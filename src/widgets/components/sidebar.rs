use std::rc::Rc;
use std::sync::{Arc, Mutex};
use relm4::adw;
use relm4::factory::FactoryVecDeque;
use relm4::{
	gtk,
	gtk::prelude::{BoxExt, OrientableExt, WidgetExt},
	ComponentParts, ComponentSender, SimpleComponent, RelmWidgetExt
};
use done_core::Channel;

use crate::{rt, fl};
use crate::widgets::factory::provider::{ProviderInput, ProviderModel};
use done_core::plugins::Plugin;
use done_core::services::provider::List;
use done_core::services::provider::provider_client::ProviderClient;

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
						gtk::CenterBox {
							set_visible: empty,
							set_orientation: gtk::Orientation::Vertical,
							set_halign: gtk::Align::Center,
							set_valign: gtk::Align::Center,
							set_vexpand: true,
							#[wrap(Some)]
							set_center_widget = &gtk::Box {
								set_orientation: gtk::Orientation::Vertical,
								set_spacing: 24,
								gtk::Picture {
									set_resource: Some("/dev/edfloreshz/Done/icons/scalable/actions/leaf.png"),
									set_margin_all: 25
								},
								gtk::Label {
									set_label: fl!("empty-sidebar"),
									set_css_classes: &["title-4", "accent"],
									set_wrap: true
								},
								gtk::Label {
									set_label: fl!("open-preferences"),
									set_wrap: true
								}
							}
						}
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
		let plugins = Plugin::list();
		let empty = !plugins.iter().any(|provider| rt().block_on(provider.connect()).is_ok());
		let widgets = view_output!();

		let mut model = SidebarModel {
			provider_factory: FactoryVecDeque::new(
				widgets.providers_container.clone(),
				sender.input_sender(),
			),
		};

		let (tx, mut rx) = tokio::sync::mpsc::channel(4);

		let local = tokio::task::LocalSet::new();

		tokio::spawn(async move {
			for provider in plugins {
				if let Ok(connector) = provider.connect().await {
					tx.send((provider, connector)).await.unwrap()
				}
			}
		});

		// let mut pluginsx: Arc<Mutex<Vec<(Plugin, ProviderClient<Channel>)>>> = Arc::new(Mutex::new(vec![]));
		//
		// 	let data = pluginsx.clone();
		// local.run_until(async move {
		// 	tokio::task::spawn_local(async move {
		// 		while let Some((provider, connector)) = rx.recv().await {
		// 			data.lock().unwrap().push((provider, connector));
		// 		}
		// 	}).await.unwrap();
		// });
		//
		// for (provider, connector) in data.lock().unwrap().iter() {
		// 	model
		// 		.provider_factory
		// 		.guard()
		// 		.push_back((provider.clone(), connector.clone()));
		// }

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
				sender.output(SidebarOutput::ListSelected(list))
			},
			SidebarInput::Forward => sender.output(SidebarOutput::Forward),
			SidebarInput::ProviderSelected(provider) => {
				sender.output(SidebarOutput::ProviderSelected(provider))
			},
		}
	}
}
