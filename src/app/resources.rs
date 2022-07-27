use anyhow::Result;
use std::env;
use relm4::gtk::gio;

pub fn load_resources() -> Result<()> {
    debug!("Loading resources...");
    gio::resources_register_include!("compiled.gresource")?;
    Ok(())
}
