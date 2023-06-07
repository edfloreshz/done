use relm4::{
	actions::{AccelsPlus, RelmAction, RelmActionGroup},
	adw,
	gtk::prelude::ApplicationExt,
	main_adw_application,
};

relm4::new_action_group!(AppActionGroup, "app");
relm4::new_stateless_action!(QuitAction, AppActionGroup, "quit");

pub(crate) fn init() {
	let app = main_adw_application();
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
