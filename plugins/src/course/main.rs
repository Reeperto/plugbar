mod course;

fn main() {}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use chrono::Weekday;
    use timetable::time::{RecurringDate, Time};

    use crate::course::{closest_course, Course};

    #[test]
    fn test_class_lookup() {
        let now = Instant::now();
        // Takes only ~30ms to do 1000 lookups on 3 classes!
        // Testing shows this is not memory bottle-necked too much.
        // Only about 2mb of memory is used... still crazy for how
        // small all the structures are though.
        for _ in 0..1_000 {
            let classes = vec![
                Course {
                    name: String::from("MATH 13"),
                    slot: RecurringDate {
                        days: vec![Weekday::Mon, Weekday::Wed, Weekday::Fri],
                        start: Time { hour: 8, minute: 0 },
                        end: Time {
                            hour: 8,
                            minute: 50,
                        },
                    },
                },
                Course {
                    name: String::from("MATH 9"),
                    slot: RecurringDate {
                        days: vec![Weekday::Mon, Weekday::Wed, Weekday::Fri],
                        start: Time {
                            hour: 13,
                            minute: 0,
                        },
                        end: Time {
                            hour: 13,
                            minute: 50,
                        },
                    },
                },
                Course {
                    name: String::from("COMLIT 10"),
                    slot: RecurringDate {
                        days: vec![Weekday::Tue, Weekday::Thu],
                        start: Time {
                            hour: 11,
                            minute: 0,
                        },
                        end: Time {
                            hour: 12,
                            minute: 20,
                        },
                    },
                },
            ];

            let _ = closest_course(&classes).unwrap();
        }
        println!("{}", now.elapsed().as_millis());
        assert!(now.elapsed().as_millis() < 500)
    }
}
