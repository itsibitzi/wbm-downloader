use std::cmp::Ordering;

use chrono::{Datelike, Duration, NaiveDate};
use lazy_static::lazy_static;
use regex::Regex;

use crate::error::Error;

use super::{PartialDateRoundingMode, YearMonth, YearMonthDayHourMinuteSecond};

#[derive(Clone, PartialEq, Eq)]
pub struct YearMonthDay {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl YearMonthDay {
    pub fn from_str(date: &str, rounding_mode: PartialDateRoundingMode) -> anyhow::Result<Self> {
        lazy_static! {
            static ref VALID_INPUT: Regex =
                Regex::new(r"(\d{4})-?(0[1-9]|1[012])?-?(0[1-9]|[12][0-9]|3[01])?").unwrap();
        }

        let captures = VALID_INPUT.captures(date).ok_or(Error::InvalidDate)?;

        match (captures.get(1), captures.get(2), captures.get(3)) {
            (Some(year), Some(month), Some(day)) => {
                let year: i32 = year.as_str().parse()?;
                let month: u32 = month.as_str().parse()?;
                let day: u32 = day.as_str().parse()?;
                Ok(Self { year, month, day })
            }
            (Some(year), Some(month), None) => {
                let year: i32 = year.as_str().parse()?;
                let month: u32 = month.as_str().parse()?;

                let day = if rounding_mode == PartialDateRoundingMode::Floor {
                    1
                } else {
                    let day_after = if month == 12 {
                        NaiveDate::from_ymd(year + 1, 1, 1)
                    } else {
                        NaiveDate::from_ymd(year, month + 1, 1)
                    };
                    let day_date = day_after - Duration::days(1);
                    day_date.day()
                };

                Ok(Self { year, month, day })
            }
            (Some(year), None, None) => {
                let year: i32 = year.as_str().parse()?;

                let (month, day) = if rounding_mode == PartialDateRoundingMode::Floor {
                    (1, 1)
                } else {
                    (12, 31)
                };

                Ok(Self { year, month, day })
            }
            _ => Err(Error::InvalidDate.into()),
        }
    }

    pub fn to_year_month(&self) -> YearMonth {
        YearMonth {
            year: self.year,
            month: self.month,
        }
    }

    pub fn to_year_month_day_hour_minute_second(
        &self,
        rounding_mode: PartialDateRoundingMode,
    ) -> YearMonthDayHourMinuteSecond {
        let (hour, minute, second) = match rounding_mode {
            PartialDateRoundingMode::Floor => (0, 0, 0),
            PartialDateRoundingMode::Ceiling => (23, 59, 59),
        };

        YearMonthDayHourMinuteSecond::new(format!(
            "{}{:0>2}{:0>2}{:0>2}{:0>2}{:0>2}",
            self.year, self.month, self.day, hour, minute, second
        ))
    }
}

impl PartialOrd for YearMonthDay {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for YearMonthDay {
    fn cmp(&self, other: &Self) -> Ordering {
        self.year
            .cmp(&other.year)
            .then(self.month.cmp(&other.month))
            .then(self.day.cmp(&other.day))
    }
}
