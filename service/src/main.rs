#[cfg(any(target_os = "linux", target_os = "macos"))]
fn daemonize() -> Result<()> {
	use daemonize::Daemonize;
	use std::fs::File;

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

	match daemonize.start() {
		Ok(_) => {
			tokio::runtime::Builder::new_current_thread()
				.enable_all()
				.build()
				.unwrap()
				.block_on(async { handle_notifications().await })
				.unwrap();
		},
		Err(e) => eprintln!("Error, {}", e),
	}
	Ok(())
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
async fn handle_notifications() -> Result<()> {
	use done_local_storage::LocalStorage;
	use notify_rust::Notification;
	use std::time::Duration;
	use tokio::time::interval;

	use chrono::{Datelike, Local, Timelike};

	let mut ticker = interval(Duration::from_secs(1));
	let mut last_minute = Local::now().time().minute();
	let local = LocalStorage::new();
	loop {
		ticker.tick().await;
		let now = Local::now().naive_local();
		let current_minute = now.time().minute();

		if current_minute != last_minute {
			last_minute = current_minute;
			let tasks: Vec<Task> = local
				.get_all_tasks()?
				.iter()
				.filter(|task| task.reminder_date.is_some())
				.filter(|task| {
					let date = task.reminder_date.unwrap();
					println!("Reminder date: {date}");
					println!("Current date: {now}");
					let is_exact_date = date.date() == now.date()
						&& date.time().hour() == now.time().hour()
						&& date.time().minute() == current_minute;
					let is_recurrent = date.weekday() == now.weekday()
						&& date.time().hour() == 9
						&& date.time().minute() == 0;
					is_exact_date || is_recurrent
				})
				.map(|task| task.to_owned())
				.collect();

			for task in tasks {
				Notification::new()
					.summary(&task.title)
					.body(&task.notes.unwrap_or_default())
					.appname("Done")
					.icon("dev.edfloreshz.Done")
					.show()?;
			}
		}
	}
}

#[cfg(target_os = "windows")]
fn daemonize() -> Result<()> {
	// Windows-specific daemonization code
	// Use winapi or other Windows-specific APIs
	// Example:
	// ...
}

use anyhow::Result;
use done_local_storage::models::Task;

fn main() -> Result<()> {
	daemonize()
}
