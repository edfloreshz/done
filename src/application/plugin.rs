use std::process::Command;

use proto_rust::provider::provider_client::ProviderClient;
use proto_rust::provider::{Empty, List};
use anyhow::Result;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use sysinfo::{ProcessExt, System, SystemExt};
use tonic::transport::Channel;

#[derive(Debug, Clone)]
pub struct PluginData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub lists: Vec<List>,
}

#[derive(Debug, EnumIter, EnumString, Display, Copy, Clone)]
pub enum Plugin {
    Local = 7007,
    Google = 6006,
    Microsoft = 3003,
    Nextcloud = 4004,
}

impl Plugin {
    pub fn list() -> Vec<Plugin> {
        Plugin::iter().collect()
    }

    pub fn start(&self) -> Result<u32, std::io::Error> {
        let process_name = match self {
            Plugin::Local => "local-plugin",
            Plugin::Google => "google-plugin",
            Plugin::Microsoft => "microsoft-plugin",
            Plugin::Nextcloud => "nextcloud-plugin",
        };
        match Command::new(process_name).spawn() {
            Ok(child) => Ok(child.id()),
            Err(err) => Err(err),
        }
    }

    pub fn stop(&self) -> Result<()> {
        let process_name = match self {
            Plugin::Local => "local-plugin",
            Plugin::Google => "google-plugin",
            Plugin::Microsoft => "microsoft-plugin",
            Plugin::Nextcloud => "nextcloud-plugin",
        };
        let processes = System::new_all();
        match processes.processes_by_exact_name(process_name).next() {
            Some(process) => {
                if process.kill() {
                    info!("Process was killed.")
                } else {
                    error!("Failed to kill process.")
                }
            }
            None => info!("Process is not running."),
        }
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        let processes = System::new_all();
        match self {
            Plugin::Local => processes
                .processes_by_exact_name("local-plugin")
                .next()
                .is_some(),
            Plugin::Google => processes
                .processes_by_exact_name("google-plugin")
                .next()
                .is_some(),
            Plugin::Microsoft => processes
                .processes_by_exact_name("microsoft-plugin")
                .next()
                .is_some(),
            Plugin::Nextcloud => processes
                .processes_by_exact_name("nextcloud-plugin")
                .next()
                .is_some(),
        }
    }

    pub fn is_installed(&self) -> bool {
        match self {
            Plugin::Local => Command::new("local-plugin").spawn().ok().is_some(),
            Plugin::Google => Command::new("google-plugin").spawn().ok().is_some(),
            Plugin::Microsoft => Command::new("microsoft-plugin").spawn().ok().is_some(),
            Plugin::Nextcloud => Command::new("nextcloud-plugin").spawn().ok().is_some(),
        }
    }

    pub async fn connect(&self) -> Result<ProviderClient<Channel>> {
        let port = *self as i32;
        let url = format!("http://[::1]:{port}");
        let plugin = ProviderClient::connect(url).await?;
        Ok(plugin)
    }

    pub async fn connected_count() -> i64 {
        let mut count = 0;
        for plugin in Plugin::list() {
            if plugin.connect().await.is_ok() {
                count += 1;
            }
        }
        count
    }

    pub async fn data(&self) -> Result<PluginData> {
        let mut connector = self.connect().await?;
        let mut stream = connector.read_all_lists(Empty {}).await?.into_inner();
        let mut lists = vec![];
        while let Some(msg) = stream.message().await.unwrap() {
            lists.push(msg.list.unwrap());
        }
        let data = PluginData {
            id: connector.get_id(Empty {}).await?.into_inner(),
            name: connector.get_name(Empty {}).await?.into_inner(),
            description: connector.get_description(Empty {}).await?.into_inner(),
            icon: connector.get_icon_name(Empty {}).await?.into_inner(),
            lists,
        };
        Ok(data)
    }

    pub fn placeholder(&self) -> PluginData {
        match self {
            Plugin::Local => PluginData {
                id: Default::default(),
                name: "Local".to_string(),
                description: "Local tasks".to_string(),
                icon: "user-home-symbolic".to_string(),
                lists: vec![],
            },
            Plugin::Google => PluginData {
                id: Default::default(),
                name: "Google".to_string(),
                description: "Google tasks".to_string(),
                icon: "user-home-symbolic".to_string(),
                lists: vec![],
            },
            Plugin::Microsoft => PluginData {
                id: Default::default(),
                name: "Microsoft To Do".to_string(),
                description: "Microsoft To Do tasks".to_string(),
                icon: "user-home-symbolic".to_string(),
                lists: vec![],
            },
            Plugin::Nextcloud => PluginData {
                id: Default::default(),
                name: "Nextcloud".to_string(),
                description: "Nextcloud tasks".to_string(),
                icon: "user-home-symbolic".to_string(),
                lists: vec![],
            },
        }
    }
}
