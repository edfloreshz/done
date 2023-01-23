use anyhow::Result;

pub(crate) async fn init() -> Result<()> {
	// for plugin in Plugin::get_local()? {
	// 	if !plugin.is_running() {
	// 		if let Err(e) = plugin.start().await {
	// 			tracing::info!("{:?}", e);
	// 		};
	// 	}
	// }
	Ok(())
}
