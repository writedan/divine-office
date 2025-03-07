use crate::kalendar::{Celebration, Color, Identifier, Kalendar, Penance, Rank, Season};
use crate::timehelp::{Betwixt, FullName, Ordinal};
use chrono::{Datelike, NaiveDate, Weekday};

pub fn get_celebration(ly: &Kalendar, date: NaiveDate) -> Celebration {
    use Weekday::*;

    let week_num = (3 - NaiveDate::weeks_since(ly.septuagesima, date)) as u8;

    let identifiers = vec![Identifier {
        season: Season::PreLent(date <= ly.purification),
        week: week_num.to_string(),
        day: String::from(date.weekday().fullname()),
    }];

    let (name, rank) = match date.weekday() {
        Sun => (
            format!("{} Sunday before Lent", week_num.ordinal()),
            Rank::StrongSunday,
        ),
        _ => (
            format!(
                "{} in the {} Week before Lent",
                date.weekday().fullname(),
                week_num.ordinal()
            ),
            Rank::Feria,
        ),
    };

    let penance = match date.weekday() {
        Wed => Some(Penance::Fasting),
        Fri => Some(Penance::Fasting),
        _ => None,
    };

    Celebration {
        name,
        penance,
        color: Color::Violet,
        rank,
        identifiers,
    }
}
