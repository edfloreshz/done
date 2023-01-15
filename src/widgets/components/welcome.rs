use relm4::gtk;
use relm4::gtk::prelude::{BoxExt, OrientableExt, WidgetExt};
use relm4::RelmWidgetExt;
use relm4::{ComponentParts, ComponentSender, SimpleComponent};

use crate::fl;

#[derive(Debug)]
pub struct Welcome {}

#[relm4::component(pub)]
impl SimpleComponent for Welcome {
	type Input = ();
	type Output = ();
	type Init = ();
	type Widgets = WelcomeWidgets;

	view! {
			#[root]
			gtk::Stack {
					set_vexpand: true,
					set_hexpand: true,
		set_transition_duration: 250,
		set_transition_type: gtk::StackTransitionType::Crossfade,
					gtk::CenterBox {
							set_orientation: gtk::Orientation::Vertical,
							set_halign: gtk::Align::Center,
							set_valign: gtk::Align::Center,
							#[wrap(Some)]
							set_center_widget = &gtk::Box {
									set_orientation: gtk::Orientation::Vertical,
									set_margin_all: 24,
									set_spacing: 24,
									gtk::Picture {
											set_resource: Some("/dev/edfloreshz/Done/icons/scalable/actions/paper-plane.png"),
											set_margin_all: 70
									},
									gtk::Label {
											set_css_classes: &["title-2", "accent"],
											set_text: fl!("select-list")
									},
									gtk::Label {
											set_text: fl!("tasks-here")
									}
							}
					}
			}
	}

	fn init(
		_init: Self::Init,
		root: &Self::Root,
		_sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let widgets = view_output!();
		let model = Welcome {};
		ComponentParts { model, widgets }
	}
}
