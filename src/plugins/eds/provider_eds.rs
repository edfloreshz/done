use glib::ffi::GHashTable;
use relm4::adw::gio::ffi::GCancellable;
use crate::adw::gio::ffi::GIcon;
use crate::core::provider::{ToDoProvider, ProviderType};
use crate::widgets::list::List;
use crate::widgets::task::Task;

///
pub struct ProviderEds {
    task_lists: GHashTable,
    cancellable: GCancellable,
    lazy_load_id: usize
}

impl ToDoProvider for ProviderEds {
    fn get_id(&self) -> &str {
        todo!()
    }

    fn get_name(&self) -> &str {
        todo!()
    }

    fn get_provider_type(&self) -> ProviderType {
        todo!()
    }

    fn get_description(&self) -> &str {
        todo!()
    }

    fn get_enabled(&self) -> bool {
        todo!()
    }

    fn refresh(&self) {
        todo!()
    }

    fn get_icon(&self) -> GIcon {
        todo!()
    }

    fn create_task(&self, list: List, task: Task) -> anyhow::Result<Task> {
        todo!()
    }

    fn update_task(&self, task: Task) -> anyhow::Result<()> {
        todo!()
    }

    fn remove_task(&self, task: Task) -> anyhow::Result<()> {
        todo!()
    }

    fn create_task_list(&self, list: List) -> anyhow::Result<List> {
        todo!()
    }

    fn update_task_list(&self, task: List) -> anyhow::Result<()> {
        todo!()
    }

    fn remove_task_list(&self, task: List) -> anyhow::Result<()> {
        todo!()
    }
}