use anyhow::Result;
use libdmd::config::Config;

use crate::services::microsoft::service::MicrosoftService;
use crate::services::ToDoService;

pub struct Settings {}

impl Settings {
    pub fn config() -> Result<()> {
        let mut config = Config::new("do")
            .about("Do is a To Do app for Linux built with Rust and GTK.")
            .author("Eduardo Flores")
            .version("0.1.0")
            .write()?;
        if !MicrosoftService::is_token_present() {
            MicrosoftService::create_config(&mut config)?;
        }
        Ok(())
    }
}
