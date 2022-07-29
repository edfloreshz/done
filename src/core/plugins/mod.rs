pub mod local;

use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use crate::core::plugins::local::service::LocalService;
use anyhow::Result;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::core::traits::provider::{ProviderService, TaskProvider};

#[derive(Serialize, Deserialize)]
pub struct Plugins {
    local: LocalService
}

impl Plugins {
    pub fn new() -> Self {
        Self {
            local: Default::default()
        }
    }
    pub fn init(&mut self) -> Result<()> {
        debug!("Initializing services...");
        self.local = LocalService::init();
        match self.local.establish_connection() {
            Ok(_) => {
                self.local.provider.set_enabled();
            },
            Err(err) => debug!("Error: {}", err)
        }
        debug!("Services initialized...");
        Ok(())
    }
}