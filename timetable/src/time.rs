use std::ops::Add;

use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, Weekday};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug)]
pub struct DateRange {
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct RecurringDate {
    pub days: Vec<Weekday>,
    pub start: Time,
    pub end: Time,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Time {
    pub hour: u32,
    pub minute: u32,
}

impl Add<Duration> for DateRange {
    type Output = DateRange;

    fn add(self, rhs: Duration) -> Self::Output {
        DateRange {
            start: self.start + rhs,
            end: self.start + rhs,
        }
    }
}

impl DateRange {
    pub fn new(start: DateTime<Local>, end: DateTime<Local>) -> Self {
        Self { start, end }
    }

    pub fn in_range(&self, time: &DateTime<Local>) -> bool {
        if time >= &self.start && time <= &self.end {
            true
        } else {
            false
        }
    }
}

impl RecurringDate {
    /// Returns a vec of [`RecurringDate`] for the current week or the current week adjusted
    /// by an offset.
    pub fn week_to_ranges(&self, offset: u32) -> Option<Vec<DateRange>> {
        let now = chrono::Local::now();
        let mut week: Vec<DateRange> = vec![];

        for day in &self.days {
            let naive_date =
                NaiveDate::from_isoywd_opt(now.year(), now.iso_week().week() + offset, *day)?;

            let naive_start = NaiveDateTime::new(
                naive_date,
                NaiveTime::from_hms_opt(self.start.hour, self.start.minute, 0)?,
            );
            let naive_end = NaiveDateTime::new(
                naive_date,
                NaiveTime::from_hms_opt(self.end.hour, self.end.minute, 0)?,
            );

            let offset = *Local::now().offset();

            let start: DateTime<Local> = DateTime::<Local>::from_local(naive_start, offset);
            let end: DateTime<Local> = DateTime::<Local>::from_local(naive_end, offset);

            week.push(DateRange::new(start, end))
        }

        Some(week)
    }
}
