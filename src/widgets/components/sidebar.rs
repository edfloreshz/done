use relm4::adw;
use relm4::component::{
	AsyncComponentParts, AsyncComponentSender, SimpleAsyncComponent,
};
use relm4::{
	gtk,
	gtk::prelude::{BoxExt, OrientableExt, WidgetExt},
	RelmWidgetExt,
};
use std::str::FromStr;
use relm4::factory::r#async::collections::AsyncFactoryVecDeque;
use crate::fl;
use crate::widgets::factory::provider::{ProviderInput, ProviderModel};
use done_core::plugins::Plugin;
use done_core::services::provider::List;
use crate::widgets::factory::list::ListData;

#[derive(Debug)]
pub struct SidebarModel {
	provider_factory: AsyncFactoryVecDeque<ProviderModel>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum SidebarInput {
	AddListToProvider(usize, String, String),
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

#[relm4::component(pub async)]
impl SimpleAsyncComponent for SidebarModel {
	type Input = SidebarInput;
	type Output = SidebarOutput;
	type Widgets = SidebarWidgets;
	type Init = ();

	view! {
		sidebar = &gtk::Box {
			set_orientation: gtk::Orientation::Vertical,
			#[name(scroll_window)]
			gtk::ScrolledWindow {
				#[name(clamp)]
				adw::Clamp {
					#[local_ref]
					providers_container -> gtk::Box {
						set_margin_top: 5,
						set_margin_start: 10,
						set_margin_end: 10,
						set_orientation: gtk::Orientation::Vertical,
						set_spacing: 12,
						set_vexpand: true,
						set_css_classes: &["navigation-sidebar"],
						gtk::CenterBox {
							#[watch]
							set_visible: false, // TODO: Show when no provider is enabled.
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

	async fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: AsyncComponentSender<Self>,
	) -> AsyncComponentParts<Self> {
		let mut model = SidebarModel {
			provider_factory: AsyncFactoryVecDeque::new(
				gtk::Box::default(),
				sender.input_sender(),
			),
		};

		let providers_container = model.provider_factory.widget();

		let widgets = view_output!();

		for provider in Plugin::list() {
			model.provider_factory.guard().push_back(provider);
			info!("Added {:?} provider to the sidebar", provider)
		}

		AsyncComponentParts { model, widgets }
	}

	async fn update(&mut self, message: Self::Input, sender: AsyncComponentSender<Self>) {
		match message {
			SidebarInput::AddListToProvider(index, provider_id, name) => {
				match Plugin::from_str(&provider_id) {
					Ok(provider) => {
						let mut service = provider.connect().await.unwrap();
						let list = List::new(&name, &provider_id);
						let response = service.create_list(list.clone()).await.unwrap();

						if response.into_inner().successful {
							self
								.provider_factory
								.send(index, ProviderInput::AddList(ListData { data: list }));
						}
					},
					Err(err) => eprintln!("{}", err),
				}
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