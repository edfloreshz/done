use relm4::RelmApp;
use widgets::app::AppModel;

mod widgets;
mod models;

fn main() {
    let model = AppModel {};
    let app = RelmApp::new(model);
    app.run()
}
