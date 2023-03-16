use chrono::{Local, Weekday};
use timetable::time::{RecurringDate, Time};

fn main() {
    let events = RecurringDate {
        days: vec![Weekday::Mon, Weekday::Thu, Weekday::Sat],
        start: Time {
            hour: 1,
            minute: 55,
        },
        end: Time {
            hour: 14,
            minute: 35,
        },
    };

    let ranges = events.week_to_ranges(0).unwrap();

    let time = Local::now().naive_local();

    if let Some(r) = ranges.iter().find(|r| r.in_range(&time)) {
        println!("Time found in range {r:?}")
    } else {
        println!("Time not found in any range")
    };

    // let start = Instant::now();
    // for _ in 1..100_000 {
    //     let (mut ranges, mut next_ranges) = events.to_ranges().unwrap();
    //     ranges.append(&mut next_ranges);

    //     let time = Local::now().naive_local();

    //     for range in ranges {
    //         if range.in_range(&time) {
    //            // println!("Currently in range {range:?}")
    //            assert!(true)
    //         }
    //     }
    // }
    // println!("--------------");
    // println!("Time elapsed: {}", start.elapsed().as_millis())
}
