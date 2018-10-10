extern crate clap;
extern crate systemstat;
extern crate notify_rust;
extern crate job_scheduler;

use std::time::Duration;
use clap::{Arg, App};
use systemstat::{System, Platform};
use notify_rust::{Notification};
use job_scheduler::{JobScheduler, Job};

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

fn run_schedule(sys: &System, interval: u64, limit: u64, title: String, msg: String) {
    let mut sched = JobScheduler::new();
    let crontab_entry: String = "1/".to_string() + &interval.to_string() + " * * * * *";

    sched.add(Job::new(crontab_entry.parse().unwrap(), move || {
        if ! is_on_ac_power(sys) && is_bellow_seconds(sys, limit * 60) {
            notify(&title.to_string(), &msg.to_string())
        }
    }));

    loop {
        sched.tick();
        std::thread::sleep(Duration::from_millis(interval * 1000));
    }
}

static VERSION: &'static str = "0.1.0";

fn main() {
    let matches = App::new("battnotify")
        .version(VERSION)
        .author("shizonic <realtiaz@gmail.com>")
        .about("Leightweight battery low notifier")
        .arg(Arg::with_name("interval")
            .short("i")
            .long("interval")
            .value_name("INTERVAL")
            .help("Sets the interval in seconds to check the remaining battery")
            .takes_value(true))
        .arg(Arg::with_name("limit")
            .short("l")
            .long("limit")
            .value_name("LIMIT")
            .help("Sets the limit in minutes remaining on which notifier is triggered")
            .takes_value(true))
        .arg(Arg::with_name("title")
            .short("t")
            .long("title")
            .value_name("TITLE")
            .help("Sets the title of the notification message")
            .takes_value(true))
        .arg(Arg::with_name("message")
            .short("m")
            .long("message")
            .value_name("INTERVAL")
            .help("Sets the body of the notification message")
            .takes_value(true))
        .get_matches();

    let interval = matches.value_of("interval").unwrap_or("30");
    let limit = matches.value_of("limit").unwrap_or("10");
    let title = matches.value_of("title").unwrap_or("Battery critical");
    let message = format!("You only have {} minutes left!", limit);
    let msg = matches.value_of("message").unwrap_or(&message);

    let sys = System::new();


    run_schedule(&sys,
        interval.parse::<u64>().unwrap(),
        limit.parse::<u64>().unwrap(),
        title.to_string(),
        msg.to_string());

}
