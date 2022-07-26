use anyhow::Result;
use std::env;
use relm4::gtk::gio;

use crate::app::constants::PKGDATADIR;

pub fn load_resources() -> Result<()> {
    debug!("Loading resources...");
	let resources = match env::var("MESON_DEVENV") {
        Err(_) => gio::Resource::load(PKGDATADIR.to_owned() + "/done.gresource")
            .expect("Unable to find done.gresource"),
        Ok(_) => {
			let path = env::current_exe()?;
			let mut resource_path = path;
			resource_path.pop();
			resource_path.push("done.gresource");
			gio::Resource::load(&resource_path)
				.expect("Unable to find done.gresource in devenv")
        },
    };
    gio::resources_register(&resources);
    Ok(())
}
