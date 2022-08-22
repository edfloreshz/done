use anyhow::Result;
use relm4::gtk::gio;
use std::env;

pub fn load_resources() -> Result<()> {
	debug!("Loading resources...");
	gio::resources_register_include!("compiled.gresource")?;
	Ok(())
}
