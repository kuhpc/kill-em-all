use psutil::process::os::unix::ProcessExt;
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

                let cpu_usage = &process.cpu_percent().expect("Couldnt get cpu percent");
                let current_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Couldnt to calculate current time")
                    .as_secs();

                let interval = psutil::host::uptime()
                    .expect("Unable to get uptime")
                    .as_secs()
                    - &process.create_time().as_secs();

                let process_name = &process.name().unwrap_or("Unnamed".to_owned());
                let process_id = &process.uids().expect("Unable to get Id").real;

                // For testing run stress which use cpu too much.
                if let Some(_) = process_name.find("stress") {
                    dbg!(current_time, interval, process_name, process_id);
                }

                // Todo Make this so that it kills only user related process not root, admin related stuff.
                // Needs to be efficient
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
