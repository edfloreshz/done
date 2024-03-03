use gpui::{actions, AppContext, KeyBinding, Menu, MenuItem};

actions!(app, [Quit]);

fn quit(_: &Quit, cx: &mut gpui::AppContext) {
	cx.quit();
}

pub fn set_actions(cx: &mut AppContext) {
	cx.on_action(quit);
	cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
}

pub fn set_menus(cx: &mut gpui::AppContext) {
	cx.set_menus(vec![Menu {
		name: "Done",
		items: vec![MenuItem::Action {
			name: "Quit",
			action: Box::new(Quit),
			os_action: None,
		}],
	}]);
}
