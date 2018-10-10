extern crate systemstat;
extern crate notify_rust;
extern crate job_scheduler;

use std::time::Duration;
use job_scheduler::{JobScheduler, Job};
use systemstat::{System, Platform};
use notify_rust::{Notification};

fn is_on_ac_power(sys: &System) -> bool {
    match sys.on_ac_power() {
        Ok(power) => power,
        Err(err) => panic!("\nError: {}", err)
    }
}

fn is_bellow_seconds(sys: &System, sec: u64) -> bool {
    match sys.battery_life() {
        Ok(battery) => battery.remaining_time.as_secs() < sec,
        Err(err) => panic!("\nError: {}", err)
    }
}

fn notify(title: &String, msg: &String) {
    Notification::new()
        .summary(title)
        .body(msg)
        .show().unwrap();
}

fn run_schedule(sched: &mut JobScheduler, interval: u64) {
    let crontab_entry: String = "1/".to_string() + &interval.to_string() + " * * * * *";

    sched.add(Job::new(crontab_entry.parse().unwrap(), || {
        notify(&"Battery Notify".to_string(), &"ok".to_string())
    }));

    loop {
        sched.tick();
        std::thread::sleep(Duration::from_millis(interval * 1000));
    }
}

fn main() {
    let mut sched = JobScheduler::new();
    run_schedule(&mut sched, 5);
}
