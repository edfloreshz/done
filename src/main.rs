use actions::{set_actions, set_menus};
use app::Done;
use gpui::*;
use options::window_options;

mod actions;
mod app;
mod options;

fn main() {
	App::new().run(|cx: &mut AppContext| {
		cx.activate(true);
		set_actions(cx);
		set_menus(cx);
		cx.open_window(window_options(), |cx| {
			cx.new_view(|_| Done {
				title: "Done".into(),
			})
		});
	});
}
