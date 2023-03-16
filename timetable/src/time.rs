use std::ops::Add;

use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Weekday};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug)]
pub struct DateRange {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
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
    pub fn new(start: NaiveDateTime, end: NaiveDateTime) -> Self {
        Self { start, end }
    }

    pub fn in_range(&self, time: &NaiveDateTime) -> bool {
        if (self.start.timestamp()..=self.end.timestamp()).contains(&time.timestamp()) {
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
            let date =
                NaiveDate::from_isoywd_opt(now.year(), now.iso_week().week() + offset, *day)?;
            let time_start = NaiveTime::from_hms_opt(self.start.hour, self.start.minute, 0)?;
            let time_end = NaiveTime::from_hms_opt(self.end.hour, self.end.minute, 0)?;

            week.push(DateRange::new(
                NaiveDateTime::new(date, time_start),
                NaiveDateTime::new(date, time_end),
            ));
        }

        Some(week)
    }
}
