use psutil::process::processes;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
	loop {
		thread::sleep(Duration::from_secs(1));
		if let Ok(processes) = processes() {
			for process in processes {
				let mut process = match process {
					Ok(pros) => pros,
					Err(_) => continue,
				};

				let cpu_usage = &process.cpu_percent().unwrap();
				let current_time = SystemTime::now()
					.duration_since(UNIX_EPOCH)
					.unwrap()
					.as_secs();

				let interval =
					psutil::host::uptime().unwrap().as_secs() - &process.create_time().as_secs();
				let process_name = &process.name().unwrap_or("Unnamed".to_owned());

				if let Some(_) = process_name.find("stress") {
					dbg!(current_time, interval, process_name);
				}
				if *cpu_usage > 5.0 && interval > 5 * 60 {
					if let Ok(process_to_kill) = psutil::process::Process::new(process.pid()) {
						println!("Killing Process {}", process_name);
						process_to_kill.kill().ok();
					}
				}
			}
		}
	}
}
