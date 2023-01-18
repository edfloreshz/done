use gtk::prelude::GtkWindowExt;
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};
use libadwaita as adw;

use crate::{config::{APP_ID, VERSION}, fl};

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

	fn init_root() -> Self::Root {}

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
		let dialog = adw::AboutWindow::builder()
			.icon_name(APP_ID)
			.application_icon(APP_ID)
			.application_name("Done")
			.developer_name("Eduardo Flores")
			.website("Website")
			.copyright("© 2022 Eduardo Flores")
			.license_type(gtk::License::Mpl20)
			.website("https://done.edfloreshz.dev/")
			.issue_url("https://github.com/done-devs/done/issues")
			.version(VERSION)
			.translator_credits(&fl!("translator-credits"))
			.modal(true)
			.translator_credits(
				"Jürgen Benvenuti\nIsabella Breder\nSabri Ünal\nalbanobattistella"
			)
			.transient_for(&widgets.main_window)
			.developers(vec!["Eduardo Flores".into()])
			.artists(vec!["Eduardo Flores".into()])
			.documenters(vec!["Eduardo Flores".into()])
			.comments("The ultimate task management solution for seamless organization and efficiency.")
			.build();
		dialog.present();
	}
}
