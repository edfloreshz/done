use app::Done;
use gpui::*;
use options::window_options;

mod app;
mod options;

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(window_options(), |cx| {
            cx.new_view(|_cx| Done {
                title: "Done".into(),
            })
        });
    });
}
