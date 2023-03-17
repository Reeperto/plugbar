use std::path::PathBuf;

use chrono::{Duration, Local};
use serde::{Deserialize, Serialize};
use timetable::time::{DateRange, RecurringDate};
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Course {
    pub name: String,
    pub slot: RecurringDate,
}

// Obtains the nearest course in the schedule and the time to the class
pub fn get_nearest_coure(courses: &Vec<Course>) -> Option<(&Course, Duration)> {
    let now = Local::now();

    let courses_and_ranges: Vec<(&Course, Vec<DateRange>)> = courses
        .iter()
        .map(|c| {
            // Obtain current week ranges and next week ranges
            let mut ranges = c.slot.week_to_ranges(0).unwrap();
            ranges.append(&mut c.slot.week_to_ranges(1).unwrap());

            (c, ranges)
        })
        .collect();

    let mut course_and_durations: Vec<(&Course, Duration)> = vec![];

    for (course, ranges) in courses_and_ranges {
        // Check if currently in a class and return if so
        if ranges.iter().any(|r| r.in_range(&now)) {
            return Some((course, Duration::seconds(0)));
        }

        // Get the minimum duration between now and when the course next occurs
        let min_duration = ranges
            .iter()
            .filter_map(|r| {
                let dt = r.start - now;

                if dt.le(&Duration::seconds(0)) {
                    None
                } else {
                    Some(dt)
                }
            })
            .min()?;

        course_and_durations.push((course, min_duration));
    }

    // Get the course with the smallest duration and return
    return course_and_durations
        .into_iter()
        .min_by(|x, y| x.1.cmp(&y.1));
}

pub fn find_course_descriptors(search_dir: PathBuf) -> Option<Vec<PathBuf>> {
    let mut yaml_files: Vec<PathBuf> = vec![];

    for entry in WalkDir::new(search_dir).into_iter().filter_map(|e| e.ok()) {
        if let Some(ext) = entry.path().extension() {
            if ext == "yml" || ext == "yaml" {
                yaml_files.push(entry.path().to_path_buf());
            }
        }
    }

    if yaml_files.is_empty() {
        None
    } else {
        Some(yaml_files)
    }
}

#[cfg(test)]
mod tests {
    use crate::course::get_nearest_coure;
    use chrono::Weekday;
    use std::time::Instant;
    use timetable::time::{RecurringDate, Time};

    use super::Course;

    #[test]
    fn test_serialization() {
        let yaml = "
            name: MATH 13
            slot:
              days: 
                - Mon
                - Wed
                - Fri
              start:
                hour: 8
                minute: 0
              end:
                hour: 8
                minute: 50";

        let out: Course = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(
            out,
            Course {
                name: String::from("MATH 13"),
                slot: RecurringDate {
                    days: vec![Weekday::Mon, Weekday::Wed, Weekday::Fri],
                    start: Time { hour: 8, minute: 0 },
                    end: Time {
                        hour: 8,
                        minute: 50
                    }
                }
            }
        )
    }

    #[test]
    fn test_deserialization() {
        let course = Course {
            name: String::from("MATH 13"),
            slot: RecurringDate {
                days: vec![Weekday::Mon, Weekday::Wed, Weekday::Fri],
                start: Time { hour: 8, minute: 0 },
                end: Time {
                    hour: 8,
                    minute: 50,
                },
            },
        };

        let yaml = "name: MATH 13\nslot:\n  days:\n  - Mon\n  - Wed\n  - Fri\n  start:\n    hour: 8\n    minute: 0\n  end:\n    hour: 8\n    minute: 50\n";

        let out = serde_yaml::to_string(&course).unwrap();

        assert_eq!(out, yaml)
    }

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

            let _ = get_nearest_coure(&classes).unwrap();
        }
        println!("{}", now.elapsed().as_millis());
        assert!(now.elapsed().as_millis() < 500)
    }
}
