use gettextrs::gettext;
use gtk::prelude::GtkWindowExt;
use relm4::{ gtk, ComponentParts, ComponentSender, SimpleComponent };

use crate::config::{APP_ID, VERSION};

pub struct AboutDialog {}

pub struct AboutDialogWidgets {
	main_window: gtk::Window,
}

impl SimpleComponent for AboutDialog {
	type Input = ();
	type Output = ();
	type Init = gtk::Window;
	type Root = ();
	type Widgets = AboutDialogWidgets;

	fn init_root() -> Self::Root {

	}

	fn init(
		main_window: Self::Init,
		_root: &Self::Root,
		_sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = Self {};

		let widgets = AboutDialogWidgets { main_window };

		ComponentParts { model, widgets }
	}

	fn update_view(
		&self,
		widgets: &mut Self::Widgets,
		_sender: ComponentSender<Self>,
	) {
		let dialog = gtk::AboutDialog::builder()
			.logo_icon_name(APP_ID)
			.program_name("Done")
			.website_label("Website")
			.copyright("© 2022 Eduardo Flores")
			.license_type(gtk::License::Gpl20Only)
			.website("https://done.edfloreshz.dev/")
			.version(VERSION)
			.translator_credits(&gettext("translator-credits"))
			.modal(true)
			.transient_for(&widgets.main_window)
			.authors(vec!["Eduardo Flores".into()])
			.artists(vec!["Eduardo Flores".into()])
			.comments("To-do lists reimagined")
			.build();
		dialog.present();
	}
}
