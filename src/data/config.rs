use anyhow::Result;
use libdmd::config::Config;

use crate::services::microsoft::service::MicrosoftService;
use crate::services::ToDoService;

pub struct Settings {}

impl Settings {
    pub fn config() -> Result<()> {
        let mut config = Config::new("do")
            .about("Microsoft To Do Client")
            .author("Eduardo Flores")
            .version("0.1.0")
            .write()?;
        if !MicrosoftService::is_token_present() {
            MicrosoftService::create_config(&mut config)?;
        }
        Ok(())
    }
}
