use chrono::{Datelike, NaiveDate, Weekday};

pub trait Sunday {
    fn this_or_next_sunday(self) -> Option<NaiveDate>;
    fn this_or_prev_sunday(self) -> Option<NaiveDate>;

    fn next_sunday(self) -> Option<NaiveDate>;
    fn prev_sunday(self) -> Option<NaiveDate>;
}

pub trait Betwixt {
    fn is_between(self, _: NaiveDate, _: NaiveDate) -> bool;
    fn weeks_since(_: NaiveDate, _: NaiveDate) -> i64;
    fn days_since(_: NaiveDate, _: NaiveDate) -> i64;
}

pub trait Ordinal {
    fn ordinal(self) -> String;
}

pub trait FullName {
    fn fullname(self) -> &'static str;
}

impl FullName for chrono::Weekday {
    fn fullname(self) -> &'static str {
        use chrono::Weekday::*;
        match self {
            Sun => "Sunday",
            Mon => "Monday",
            Tue => "Tuesday",
            Wed => "Wednesday",
            Thu => "Thursday",
            Fri => "Friday",
            Sat => "Saturday",
        }
    }
}

impl Ordinal for i64 {
    fn ordinal(self) -> String {
        let suffix = match self % 10 {
            1 if self % 100 != 11 => "st",
            2 if self % 100 != 12 => "nd",
            3 if self % 100 != 13 => "rd",
            _ => "th",
        };

        format!("{}{}", self, suffix)
    }
}

impl Ordinal for u8 {
    fn ordinal(self) -> String {
        let suffix = match self % 10 {
            1 if self % 100 != 11 => "st",
            2 if self % 100 != 12 => "nd",
            3 if self % 100 != 13 => "rd",
            _ => "th",
        };

        format!("{}{}", self, suffix)
    }
}

impl Sunday for NaiveDate {
    fn this_or_next_sunday(self) -> Option<NaiveDate> {
        use Weekday::*;
        let days_to_add = match self.weekday() {
            Mon => 6,
            Tue => 5,
            Wed => 4,
            Thu => 3,
            Fri => 2,
            Sat => 1,
            Sun => 0,
        };

        self.checked_add_signed(chrono::Duration::days(days_to_add))
    }

    fn this_or_prev_sunday(self) -> Option<NaiveDate> {
        use Weekday::*;
        let days_to_sub = match self.weekday() {
            Sun => 0,
            Mon => 1,
            Tue => 2,
            Wed => 3,
            Thu => 4,
            Fri => 5,
            Sat => 6,
        };

        self.checked_sub_signed(chrono::Duration::days(days_to_sub))
    }

    fn next_sunday(self) -> Option<NaiveDate> {
        let sunday = self.this_or_next_sunday()?;
        if sunday == self {
            sunday.checked_add_signed(chrono::Duration::days(7))
        } else {
            Some(sunday)
        }
    }

    fn prev_sunday(self) -> Option<NaiveDate> {
        let sunday = self.this_or_prev_sunday()?;
        if sunday == self {
            sunday.checked_sub_signed(chrono::Duration::days(7))
        } else {
            Some(sunday)
        }
    }
}

impl Betwixt for NaiveDate {
    fn is_between(self, d1: NaiveDate, d2: NaiveDate) -> bool {
        self >= d1 && self < d2 // on or after start date, but before end date
    }

    fn weeks_since(d1: NaiveDate, d2: NaiveDate) -> i64 {
        (d2 - d1).num_days() / 7
    }

    fn days_since(d1: NaiveDate, d2: NaiveDate) -> i64 {
        (d2 - d1).num_days()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weeks_since() {
        let d1 = NaiveDate::from_ymd(2024, 1, 1).prev_sunday().unwrap();
        let mut d2 = d1.next_sunday().unwrap();
        for x in 1..=52 {
            println!("{:?} weeks since {:?} = {}", d2, d1, x);
            assert_eq!(NaiveDate::weeks_since(d1, d2), x);
            d2 = d2.next_sunday().unwrap();
        }
    }
}
