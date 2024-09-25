//! The Wayback machine has various dates and date times in it's API. This
//! module contains those.mod year_month;

mod year_month;
mod year_month_day;
mod year_month_day_hour_minute_second;

pub use year_month::YearMonth;
pub use year_month_day::YearMonthDay;
pub use year_month_day_hour_minute_second::YearMonthDayHourMinuteSecond;

/// Defines how partial dates should be rounded.
/// E.g. if the user provides 2020-04 and we floor then
/// the output would be 2020-04-01, if we ceiling the date
/// then it would be 2020-04-30.
#[derive(PartialEq)]
pub enum PartialDateRoundingMode {
    /// Used when parsing 'from' dates
    Floor,
    /// Used when parsing 'until' dates
    Ceiling,
}
