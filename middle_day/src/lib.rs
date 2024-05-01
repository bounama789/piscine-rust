use chrono::{Datelike, NaiveDate, Weekday as wd};

pub fn middle_day(year: i32) -> Option<wd> {
    let is_leap_year = NaiveDate::from_ymd_opt(year, 2, 29).is_some();
    if is_leap_year {
        None
    } else {
        let days_in_year =
            NaiveDate::from_ymd_opt(year, 12, 31).and_then(|date| Some(date.ordinal()));
        if let Some(days) = days_in_year {
            if days % 2 == 0 {
                None
            } else {
                let date = NaiveDate::from_ymd_opt(year, 1, 1)
                    .unwrap()
                    .with_ordinal((days / 2) + 1)
                    .unwrap();
                Some(date.weekday())
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_middle_day_non_leap_year() {
        let year = 1022; // 2021 is not a leap year
        assert_eq!(middle_day(year), Some(wd::Tue));
    }

    #[test]
    fn test_middle_day_leap_year() {
        let year = 2020; // 2020 is a leap year
        assert_eq!(middle_day(year), None);
    }
}
