use std::{cmp::Ordering, collections::BTreeSet, fmt};

use crate::error::Error;

#[derive(Clone, PartialEq, Eq)]
pub struct YearMonth {
    pub year: i32,
    pub month: u32,
}

impl YearMonth {
    pub fn months_between_inclusive(&self, other: &Self) -> anyhow::Result<BTreeSet<YearMonth>> {
        if self > other {
            return Err(Error::DateRangeBackwards.into());
        }

        let mut results = BTreeSet::new();

        let start = self;
        let end = other;

        let mut current = YearMonth {
            year: start.year,
            month: start.month,
        };

        while current.year < end.year || (current.year == end.year && current.month <= end.month) {
            results.insert(current.clone());

            // Move to the next month
            current.month += 1;
            if current.month > 12 {
                current.year += 1;
                current.month = 1;
            }
        }

        Ok(results)
    }

    pub fn to_wbm_date_string(&self) -> String {
        format!("{}{:0>2}", self.year, self.month)
    }
}

impl fmt::Display for YearMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{:0>2}", self.year, self.month)
    }
}

impl PartialOrd for YearMonth {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for YearMonth {
    fn cmp(&self, other: &Self) -> Ordering {
        self.year
            .cmp(&other.year)
            .then(self.month.cmp(&other.month))
    }
}
