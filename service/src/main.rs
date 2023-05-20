#[cfg(any(target_os = "linux", target_os = "macos"))]
async fn daemonize() -> Result<()> {
	use std::{fs::File, time::Duration};

	use chrono::{Local, Timelike};
	use daemonize::Daemonize;
	use done_local_storage::{models::Task, LocalStorage};
	use notify_rust::Notification;
	use tokio::time::interval;

	let stdout = File::create("/tmp/done_service.log").unwrap();
	let stderr = File::create("/tmp/done_service.err").unwrap();

	let daemonize = Daemonize::new()
		.pid_file("/tmp/done_service.pid")
		.chown_pid_file(true)
		.working_directory("/tmp")
		.umask(0o027)
		.stdout(stdout)
		.stderr(stderr)
		.privileged_action(|| {
			// Executed before drop privileges
			println!("Running privileged action before dropping privileges.");
		});

	// loop {
	// 	ticker.tick().await;

	// 	let current_minute = Local::now().time().minute();
	// 	if current_minute != last_minute {
	// 		last_minute = current_minute;

	// 		// Your code to be executed every time the minute changes
	// 		println!("Current time: {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
	// 	}
	// }

	match daemonize.start() {
		Ok(_) => {
			let mut ticker = interval(Duration::from_secs(1));
			let mut last_minute = Local::now().time().minute();

			loop {
				ticker.tick().await;
				println!("Tick...");

				let now = Local::now().naive_local();

				let current_minute = now.time().minute();
				if current_minute != last_minute {
					println!("Current time: {}", now.format("%Y-%m-%d %H:%M:%S"));
					println!("Current minute: {}", current_minute);
					println!("Last minute: {}", last_minute);
					last_minute = current_minute;
				}
			}
		},
		Err(e) => eprintln!("Error, {}", e),
	}
	Ok(())
}

#[cfg(target_os = "windows")]
fn daemonize() -> Result<()> {
	// Windows-specific daemonization code
	// Use winapi or other Windows-specific APIs
	// Example:
	// ...
}

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
	daemonize().await
}
