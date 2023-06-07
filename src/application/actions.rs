use std::str::FromStr;

use done_local_storage::service::Service;
use relm4::actions::{AccelsPlus, RelmAction, RelmActionGroup};
use relm4::adw;
use relm4::adw::prelude::ApplicationExt;
use relm4::gtk::gio::ApplicationFlags;
use relm4::gtk::prelude::{ApplicationExtManual, FileExt};

relm4::new_action_group!(AppActionGroup, "app");
relm4::new_stateless_action!(QuitAction, AppActionGroup, "quit");

pub(crate) fn init(app: &adw::Application) {
	app.set_flags(ApplicationFlags::HANDLES_OPEN);

	app.connect_open(|_, files, _| {
		println!("Yay!");
		let bytes = files[0].uri();
		let uri = reqwest::Url::from_str(bytes.to_string().as_str()).unwrap();
		relm4::tokio::spawn(async move {
			let pairs = uri.query_pairs();
			let response = Service::Microsoft
				.get_service()
				.handle_uri_params(pairs)
				.await;
			match response {
				Ok(_) => tracing::info!("Token stored"),
				Err(err) => tracing::error!("An error ocurred: {}", err),
			}
		});
	});

	app.set_resource_base_path(Some("/dev/edfloreshz/Done/"));
	let mut actions = RelmActionGroup::<AppActionGroup>::new();

	let quit_action = {
		let app = app.clone();
		RelmAction::<QuitAction>::new_stateless(move |_| {
			app.quit();
		})
	};

	actions.add_action(quit_action);

	app.set_accelerators_for_action::<QuitAction>(&["<Control>q"]);

	app.set_action_group(Some(&actions.into_action_group()));
}
