use crate::adw::gio::ffi::GIcon;
use crate::core::provider::{ToDoProvider, ProviderType};
use crate::plugins::eds::provider_eds::ProviderEds;
use crate::widgets::list::List;
use crate::widgets::task::Task;

pub struct ProviderLocal {
    parent: ProviderEds,
    icon: GIcon,
    task_lists: Vec<List>
}

impl ToDoProvider for ProviderLocal {
    fn get_id(&self) -> &str {
        "locale"
    }

    fn get_name(&self) -> &str {
        "On This Computer"
    }

    fn get_provider_type(&self) -> ProviderType {
        ProviderType::Local
    }

    fn get_description(&self) -> &str {
        "Local"
    }

    fn get_enabled(&self) -> bool {
        true
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