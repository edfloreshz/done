use relm4::RelmApp;
use widgets::app::AppModel;
use crate::models::list::List;

mod widgets;
mod models;

fn main() {
    let model = AppModel {
        lists: vec![
            List {
                task_list_id: "1".to_string(),
                display_name: "Test".to_string(),
                is_owner: false,
                is_shared: false
            }
        ]
    };
    let relm = RelmApp::new(model);
    relm.run()
}
