use super::YearMonth;

/// Represents a full timestamp to second precision.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct YearMonthDayHourMinuteSecond(
    // This type is only really used as an ID for captures
    // so there's not much value in parsing them
    String,
);

impl YearMonthDayHourMinuteSecond {
    pub fn new(v: String) -> Self {
        Self(v)
    }

    pub fn from_ym_and_dhms_as_u32(ym: &YearMonth, i: u32) -> Self {
        // The WBM returns integers so leading zeros are cut off.
        // E.g. A capture on the 2nd day of a given month at 12:34:56
        // would be 2123456. The same time on the 12th would be
        // 12123456. When looking up the specific capture we need a leading
        // zero for they day value.

        let padded_string = if i > 9_99_99_99 {
            format!("{}{:0>2}{}", ym.year, ym.month, i)
        } else {
            format!("{}{:0>2}0{}", ym.year, ym.month, i)
        };

        Self(padded_string)
    }

    pub fn as_wbm_datetime_str(&self) -> &str {
        &self.0
    }
}
