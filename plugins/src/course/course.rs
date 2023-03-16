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

#[allow(dead_code)]
pub fn closest_course(courses: &Vec<Course>) -> Option<(&Course, Duration)> {
    let now = Local::now();
    let now_epoch = now.timestamp();

    // Steps to find nearest course
    // 1. Associate every course with the next 2 weeks of date ranges
    // 2. Check if currently in class. If so return class. Else:
    // 3. Convert all to unix epoch form based on their start time
    // 4. Take min of unix epochs
    // 5. Return class that is nearest alongside duration to it.

    let ranges: Vec<(&Course, Vec<DateRange>)> = courses
        .iter()
        .map(|c| {
            // Obtain current week ranges and next week ranges
            let mut ranges = c.slot.week_to_ranges(0).unwrap();
            ranges.append(&mut c.slot.week_to_ranges(1).unwrap());

            (c, ranges)
        })
        .collect();

    for (course, rs) in &ranges {
        for r in rs {
            if r.in_range(&now.naive_local()) {
                return Some((course, Duration::seconds(0)));
            }
        }
    }

    let epochs: Vec<(&Course, i64)> = ranges
        .iter()
        .map(|(c, rs)| {
            (
                *c,
                rs.iter()
                    .map(|r| {
                        let start = r.start.timestamp();

                        if r.end.timestamp() < now_epoch {
                            i64::MAX
                        } else {
                            start
                        }
                    })
                    .min()
                    .unwrap(),
            )
        })
        .collect();
    
    if let Some(e) = epochs.iter().min_by(|(_, t1), (_, t2)| t1.cmp(t2)) {
        return Some((e.0, Duration::seconds(e.1 - now_epoch)));
    } else {
        return None
    }
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
    use chrono::Weekday;
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
}
