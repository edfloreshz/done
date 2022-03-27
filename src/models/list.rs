use gtk4 as gtk;
use gtk::prelude::*;
use relm4_macros::view;
use serde::{Deserialize, Serialize};
use crate::BaseWidgets;

#[allow(dead_code)]
pub enum ListMsg {
    Delete(usize),
    Create(String),
    Select(usize),
    Rename(usize, String),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct List {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "isOwner")]
    pub is_owner: bool,
    #[serde(rename = "isShared")]
    pub is_shared: bool,
    #[serde(rename = "id")]
    pub task_list_id: String,
}

impl List {
    pub fn fill_lists(ui: &BaseWidgets, data: &Vec<List>) {
        for list in data.iter() {
            view! {
            label = &gtk::Label {
                set_text: list.display_name.as_str(),
                set_height_request: 40,
            }
        }
            ui.sidebar.list.append(&label);
        }
    }
}