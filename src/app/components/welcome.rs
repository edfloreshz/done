use crate::fl;
use relm4::gtk::prelude::{BoxExt, OrientableExt, WidgetExt};
use relm4::{adw, gtk, RelmWidgetExt};
use relm4::{ComponentParts, ComponentSender, SimpleComponent};

#[derive(Debug)]
pub struct WelcomeComponent;

#[relm4::component(pub)]
impl SimpleComponent for WelcomeComponent {
	type Input = ();
	type Output = ();
	type Init = ();

	view! {
		#[root]
		adw::Clamp {
			set_maximum_size: 450,
			gtk::Box {
				set_orientation: gtk::Orientation::Vertical,
				set_vexpand: true,
				set_hexpand: true,
				set_valign: gtk::Align::Center,
				set_halign: gtk::Align::Center,
				set_spacing: 10,
				set_margin_all: 20,
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
					set_vexpand: true,
					set_resource: Some("/dev/edfloreshz/Done/icons/scalable/apps/app-icon.svg"),
					set_content_fit: gtk::ContentFit::ScaleDown,
				},
				gtk::Label {
					set_css_classes: &["body"],
					set_wrap: true,
					set_wrap_mode: gtk::pango::WrapMode::Word,
					set_justify: gtk::Justification::Center,
					set_text: fl!("welcome-body")
				},
			}
		}
	}

	fn init(
		_init: Self::Init,
		root: Self::Root,
		_sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let widgets = view_output!();
		let model = WelcomeComponent {};
		ComponentParts { model, widgets }
	}
}
