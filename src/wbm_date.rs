use std::collections::HashSet;

use chrono::{naive::NaiveDate, Datelike, Duration};
use lazy_static::lazy_static;
use regex::Regex;

use crate::error::Error;

#[derive(PartialEq)]
pub enum RangePart {
    From,
    Until
}

pub fn to_wbm_date(range_part: RangePart, date: String) -> anyhow::Result<String> {
    lazy_static! {
        static ref VALID_INPUT: Regex = Regex::new(r"(\d{4})-?(0[1-9]|1[012])?-?(0[1-9]|[12][0-9]|3[01])?").unwrap();
    }

    let captures = VALID_INPUT.captures(&date).ok_or(Error::InvalidDate)?;

    match (captures.get(1), captures.get(2), captures.get(3)) {
        (Some(year), Some(month), Some(day)) => {
            let mut wbm_date = String::from(year.as_str());
            wbm_date.push_str(month.as_str());
            wbm_date.push_str(day.as_str());
            Ok(wbm_date)
        },
        (Some(year), Some(month), None) => {
            let mut wbm_date = String::from(year.as_str());
            wbm_date.push_str(month.as_str());

            // The regex should prevent this from panicing since the max value is 9999
            let year_num = year.as_str().parse::<i32>().unwrap();
            // Similar as the year, the regex should prevent illegal days
            let month_num = month.as_str().parse::<u32>().unwrap();

            let day = if range_part == RangePart::From {
                let d = NaiveDate::from_ymd(year_num, month_num, 1);
                d.day()
            } else {
                let d = if month_num == 12 {
                    NaiveDate::from_ymd(year_num + 1, 1, 1)
                } else {
                    NaiveDate::from_ymd(year_num, month_num + 1, 1)
                };
                let previous_date = d - Duration::days(1);
                previous_date.day()
            };

            wbm_date.push_str(&day.to_string());
            Ok(wbm_date)
        },
        (Some(year), None, None) => {
            let mut wbm_date = String::from(year.as_str());

            if range_part == RangePart::From {
                wbm_date.push_str("0101");
            } else {
                wbm_date.push_str("1231");
            }

            Ok(wbm_date)
        },
        _ => Err(Error::InvalidDate.into())
    }
}

pub fn get_months_between_dates(from: &str, until: &str) -> HashSet<String> {
    // All unwraps because the format converter
    let from_y = from[..4].parse::<i32>().unwrap();
    let from_m = from[4..6].parse::<i32>().unwrap();

    let until_y = until[..4].parse::<i32>().unwrap();
    let until_m = until[4..6].parse::<i32>().unwrap();

    let mut output = HashSet::new();

    if from_y == until_y {
        for month in from_m..=until_m {
            let mut ym = from_y.to_string();

            let m = format!("{:02}", month);
            ym.push_str(&m);

            output.insert(ym);
        }
    } else {
        // First year months
        for month in from_m..=12 {
            let mut ym = from_y.to_string();

            let m = format!("{:02}", month);
            ym.push_str(&m);

            output.insert(ym);
        }

        // Middle whole years...
        for year in from_y + 1..until_y {
            for month in 1..=12 {
                let mut ym = year.to_string();

                let m = format!("{:02}", month);
                ym.push_str(&m);

                output.insert(ym);
            }
        }

        // Last year months
        for month in 1..=until_m {
            let mut ym = until_y.to_string();

            let m = format!("{:02}", month);
            ym.push_str(&m);

            output.insert(ym);
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::wbm_date::get_months_between_dates;

    #[test]
    fn month_range_same_month() {
        let expected = HashSet::from(["202203"].map(|ym| ym.to_string()));
        let actual = get_months_between_dates("20220301", "20220320");
        assert_eq!(expected, actual);
    }

    #[test]
    fn month_range_within_a_year() {
        let expected = HashSet::from(["202201", "202202", "202203"].map(|ym| ym.to_string()));
        let actual = get_months_between_dates("20220101", "20220301");
        assert_eq!(expected, actual);
    }

    #[test]
    fn month_range_within_across_one_year() {
        let expected = HashSet::from(["202212", "202301"].map(|ym| ym.to_string()));
        let actual = get_months_between_dates("20221201", "20230101");
        assert_eq!(expected, actual);
    }

    #[test]
    fn month_range_within_across_two_years() {
        let expected = HashSet::from(
            [
                "202211", "202212", "202301", "202302", "202303", "202304", "202305", "202306",
                "202307", "202308", "202309", "202310", "202311", "202312", "202401", "202402",
            ]
            .map(|ym| ym.to_string()),
        );
        let actual = get_months_between_dates("20221101", "20240201");
        assert_eq!(expected, actual);
    }
}