use std::fs;

use course::{Course, get_nearest_coure};

mod course;

fn main() {
    let classes: Vec<Course> = serde_yaml::from_str(&fs::read_to_string("./example_schedule.yml").unwrap()).unwrap();

    let (class, dur) = get_nearest_coure(&classes).unwrap();

    if dur.num_seconds() == 0 {
        println!("Currently in {}", class.name)
    } else {
        println!("{} in {} hrs, {} mins", class.name, dur.num_hours(), dur.num_minutes() % 60)
    }
}

