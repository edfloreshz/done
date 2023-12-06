use log::info;

pub fn setup() {
	env_logger::init();
	info!("Starting logger...");
}
