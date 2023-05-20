use relm4::gtk;
use relm4::gtk::prelude::{BoxExt, OrientableExt, WidgetExt};
use relm4::RelmWidgetExt;
use relm4::{ComponentParts, ComponentSender, SimpleComponent};

use crate::fl;

#[derive(Debug)]
pub struct WelcomeComponent;

#[relm4::component(pub)]
impl SimpleComponent for WelcomeComponent {
	type Input = ();
	type Output = ();
	type Init = ();

	view! {
		#[root]
		gtk::Stack {
			set_transition_duration: 250,
			set_transition_type: gtk::StackTransitionType::Crossfade,
			gtk::Box {
				set_vexpand: true,
				set_hexpand: true,
				set_orientation: gtk::Orientation::Vertical,
				set_halign: gtk::Align::Center,
				set_valign: gtk::Align::Center,
				set_margin_all: 50,
				set_spacing: 20,
				gtk::Label {
					set_css_classes: &["title-1"],
					set_wrap: true,
					set_wrap_mode: gtk::pango::WrapMode::Word,
					set_justify: gtk::Justification::Center,
					set_text: fl!("welcome-title")
				},
				gtk::Label {
					set_css_classes: &["title-4"],
					set_wrap: true,
					set_wrap_mode: gtk::pango::WrapMode::Word,
					set_justify: gtk::Justification::Center,
					set_text: fl!("welcome-subtitle")
				},
				gtk::Picture {
					set_resource: Some("/dev/edfloreshz/Done/icons/scalable/apps/app-icon.svg"),
					set_content_fit: gtk::ContentFit::ScaleDown,
				},
				gtk::Label {
					set_css_classes: &["body"],
					set_wrap: true,
					set_wrap_mode: gtk::pango::WrapMode::Word,
					set_justify: gtk::Justification::Center,
					set_text: fl!("welcome-body")
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
		let model = WelcomeComponent {};
		ComponentParts { model, widgets }
	}
}
