#[cfg(target_os = "macos")]
pub fn setup() {
	match notify_rust::set_application(environment::APPLICATION_ID) {
		Ok(_) => {},
		Err(error) => {
			log::error!("{}", error.to_string());
		},
	}
}

#[cfg(not(target_os = "macos"))]
pub fn setup() {}
