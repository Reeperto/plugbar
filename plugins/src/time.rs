use std::{env, process::Command};

use chrono::Local;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let cmd = &args[1];
        match &cmd[..] {
            "-t" => set_time(false),
            "-d" => set_time(true),
            _ => (),
        };
    }
}

fn set_time(day: bool) {
    let name = env::var("NAME").unwrap_or("".to_string());
    Command::new("sketchybar")
        .args(["--set", &name])
        .arg(get_time(day))
        .spawn()
        .unwrap();
}

fn get_time(day: bool) -> String {
    let time = Local::now();
    if day {
        format!("icon={}", time.format("%B %-e"))
    } else {
        format!("icon={}", time.format("%-l:%M:%S %p"))
    }
}
