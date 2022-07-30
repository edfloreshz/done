use crate::gtk;
use crate::gtk::Image;
use serde::{Serialize, Deserialize};
use crate::core::traits::provider::{ProviderType, TaskProvider};

pub mod service;
pub mod models;

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalProvider {
    id: String,
    name: String,
    provider_type: ProviderType,
    description: String,
    enabled: bool,
    #[serde(skip)]
    icon: String,
}

impl Default for LocalProvider {
    fn default() -> Self {
        Self {
            id: "local".to_string(),
            name: "Local".to_string(),
            provider_type: ProviderType::Local,
            description: "Local storage".to_string(),
            enabled: false,
            icon: Default::default()
        }
    }
}

impl TaskProvider for LocalProvider {
    fn get_id(&self) -> &str {
        &*self.id
    }

    fn get_name(&self) -> &str {
        &*self.name
    }

    fn get_provider_type(&self) -> ProviderType {
        self.provider_type.clone()
    }

    fn get_description(&self) -> &str {
        &*self.description
    }

    fn get_enabled(&self) -> bool {
        self.enabled
    }
    fn set_enabled(&mut self) {
        self.enabled = true
    }

    fn refresh(&self) {
        todo!()
    }

    fn get_icon(&self) -> Image {
        Image::from_resource(self.icon.as_str())
    }
}