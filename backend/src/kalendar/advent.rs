use crate::kalendar::{Celebration, Color, Identifier, Kalendar, Penance, Rank, Season};
use crate::timehelp::{Betwixt, FullName, Ordinal};
use chrono::{Datelike, NaiveDate, Weekday};

pub fn get_celebration(ly: &Kalendar, date: NaiveDate) -> Celebration {
    let week_num = (NaiveDate::weeks_since(ly.advent, date) + 1) as u8;
    let weekday = date.weekday();
    let weekday_name = weekday.fullname();
    let is_ember_week = week_num == 3;

    let (name, color, penance, rank) = match weekday {
        Weekday::Sun => (
            format!("{} Sunday of Advent", week_num.ordinal()),
            if is_ember_week {
                Color::Rose
            } else {
                Color::Violet
            },
            None,
            Rank::StrongSunday,
        ),

        Weekday::Wed | Weekday::Fri if is_ember_week => (
            format!("Ember {} of Advent", weekday_name),
            Color::Violet,
            Some(Penance::Fasting),
            Rank::Feria,
        ),

        Weekday::Sat => {
            let name = if is_ember_week {
                String::from("Ember Saturday of Advent")
            } else {
                format!("Saturday in the {} Week of Advent", week_num.ordinal())
            };

            let penance = if is_ember_week {
                Penance::Vigil
            } else {
                Penance::Fasting
            };

            (name, Color::Violet, Some(penance), if is_ember_week { Rank::Eve } else { Rank::Feria })
        }

        _ => {
            let penance = match weekday {
                Weekday::Wed | Weekday::Fri => Some(Penance::Fasting),
                _ => None,
            };

            (
                format!(
                    "{} in the {} Week of Advent",
                    weekday_name,
                    week_num.ordinal()
                ),
                Color::Violet,
                penance,
                Rank::Feria,
            )
        }
    };

    let mut identifiers = vec![Identifier {
        season: Season::Advent,
        week: week_num.to_string(),
        day: String::from(weekday_name),
        weekday,
    }];

    let o_wisdom = NaiveDate::from_ymd_opt(date.year(), 12, 17).unwrap();
    if date >= o_wisdom {
        identifiers.push(Identifier {
            season: Season::Advent,
            week: String::from("o-antiphons"),
            day: (NaiveDate::days_since(o_wisdom, date) + 1).to_string(),
            weekday,
        });
    }

    Celebration {
        name,
        color,
        penance,
        rank,
        identifiers,
    }
}
