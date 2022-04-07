use relm4::RelmApp;
use widgets::app::AppModel;

mod widgets;
mod models;

fn main() {
    let model = AppModel {
        lists: vec![]
    };
    let relm = RelmApp::new(model);
    relm.run()
}
