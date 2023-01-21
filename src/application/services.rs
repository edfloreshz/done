use anyhow::Result;

use super::plugin::Plugin;

pub(crate) async fn init() -> Result<()> {
	relm4::spawn(async move {
		for plugin in Plugin::fetch_plugins().await.unwrap() {
			if !plugin.is_running() {
				if let Err(e) = plugin.start() {
					tracing::info!("{:?}", e);
				};
			}
		}
	})
	.await?;
	Ok(())
}
