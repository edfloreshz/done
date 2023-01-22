use anyhow::Result;

use super::plugin::Plugin;

pub(crate) async fn init() -> Result<()> {
	for plugin in Plugin::fetch_plugins().await.unwrap() {
		if !plugin.is_running() {
			if let Err(e) = plugin.start() {
				tracing::info!("{:?}", e);
			};
		}
	}
	Ok(())
}
