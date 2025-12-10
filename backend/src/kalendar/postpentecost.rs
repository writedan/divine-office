use crate::kalendar::{Celebration, Color, Identifier, Kalendar, Penance, Rank, Season};
use crate::timehelp::{Betwixt, FullName, Ordinal, Sunday};
use chrono::{Datelike, NaiveDate, Weekday};

pub fn get_celebration(ly: &Kalendar, date: NaiveDate) -> Celebration {
    use Weekday::*;

    let sundays_after_pentecost =
        NaiveDate::weeks_since(ly.pentecost.next_sunday().unwrap(), ly.next_advent);
    let sunday_num = NaiveDate::weeks_since(ly.pentecost, date);
    let week_num = if sunday_num == sundays_after_pentecost {
        28
    } else {
        sunday_num
    } as u8;

    let weekday = date.weekday();
    let weekday_name = weekday.fullname();

    let mut identifiers = vec![Identifier {
        season: if (24..28).contains(&week_num) {
            Season::PostEpiphany(date <= ly.purification)
        } else {
            Season::PostPentecost
        },
        week: if (24..28).contains(&week_num) {
            ((week_num - 24) + 1).to_string()
        } else {
            week_num.to_string()
        },
        day: String::from(weekday_name),
        weekday,
    }];

    let aug_sunday = NaiveDate::from_ymd_opt(date.year(), 7, 28)
        .unwrap()
        .next_sunday()
        .unwrap();

    if date < aug_sunday {
        let (name, rank) = match weekday {
            Sun => (
                format!("{} Sunday after Pentecost", week_num.ordinal()),
                Rank::Sunday,
            ),
            _ => (
                format!(
                    "{} in the {} Week after Pentecost",
                    weekday_name,
                    week_num.ordinal()
                ),
                Rank::Feria,
            ),
        };

        let penance = match weekday {
            Wed | Fri => Some(Penance::Abstinence),
            _ => None,
        };

        return Celebration {
            name,
            penance,
            color: Color::Green,
            rank,
            identifiers,
        };
    }

    let sep_sunday = NaiveDate::from_ymd_opt(date.year(), 8, 28)
        .unwrap()
        .next_sunday()
        .unwrap();
    let oct_sunday = NaiveDate::from_ymd_opt(date.year(), 9, 27)
        .unwrap()
        .next_sunday()
        .unwrap();
    let nov_sunday = NaiveDate::from_ymd_opt(date.year(), 10, 28)
        .unwrap()
        .next_sunday()
        .unwrap();

    let sep_weeks = NaiveDate::weeks_since(sep_sunday, oct_sunday);

    let (month_week_num, month, month_season) = if date.is_between(aug_sunday, sep_sunday) {
        (
            NaiveDate::weeks_since(aug_sunday, date) + 1,
            "August",
            Season::August,
        )
    } else if date.is_between(sep_sunday, oct_sunday) {
        let sep_week_num = NaiveDate::weeks_since(sep_sunday, date) + 1;
        let adjusted_week = if sep_weeks == 4 && sep_week_num == 4 {
            sep_week_num + 1
        } else {
            sep_week_num
        };
        (adjusted_week, "September", Season::September)
    } else if date.is_between(oct_sunday, nov_sunday) {
        (
            NaiveDate::weeks_since(oct_sunday, date) + 1,
            "October",
            Season::October,
        )
    } else {
        let nov_week_num = NaiveDate::weeks_since(nov_sunday, date) + 1;
        let adjusted_week = if (2..=4).contains(&nov_week_num) {
            nov_week_num + 1
        } else {
            nov_week_num
        };
        (adjusted_week, "November", Season::November)
    };

    let is_ember_day = month == "September" && month_week_num == 3;

    let name = match (week_num, weekday) {
        (28, Sun) => format!(
            "Last Sunday after Pentecost and {} in {}",
            month_week_num.ordinal(),
            month
        ),
        (28, _) => format!(
            "{} in the Last Week after Pentecost and {} in {}",
            weekday_name,
            month_week_num.ordinal(),
            month
        ),
        (_, Sun) => format!(
            "{} Sunday after Pentecost and {} in {}",
            week_num.ordinal(),
            month_week_num.ordinal(),
            month
        ),
        (_, Wed | Fri | Sat) if is_ember_day => {
            format!("Ember {} of September", weekday_name)
        }
        (_, _) => format!(
            "{} in the {} Week after Pentecost and {} in {}",
            weekday_name,
            week_num.ordinal(),
            month_week_num.ordinal(),
            month
        ),
    };

    let (color, penance) = match (weekday, is_ember_day) {
        (Wed | Fri | Sat, true) => (
            Color::Violet,
            Some(match weekday {
                Sat => Penance::Vigil,
                _ => Penance::Fasting,
            }),
        ),
        (Wed | Fri, false) => (Color::Green, Some(Penance::Abstinence)),
        _ => (Color::Green, None),
    };

    let rank = if weekday == Sun {
        Rank::Sunday
    } else {
        Rank::Feria
    };

    identifiers.push(Identifier {
        season: month_season,
        week: month_week_num.to_string(),
        day: String::from(weekday_name),
        weekday,
    });

    Celebration {
        name,
        color,
        rank,
        penance,
        identifiers,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sunday_diff() {
        for x in 1600..=6000 {
            let ly = Kalendar::from_year(x).unwrap();
            let sundays_after_pentecost =
                NaiveDate::weeks_since(ly.pentecost.next_sunday().unwrap(), ly.next_advent);
            let sundays_after_epiphany =
                NaiveDate::weeks_since(ly.epiphany_sunday, ly.septuagesima);

            assert!((sundays_after_pentecost - 24) <= (6 - sundays_after_epiphany), "year {} had {} sundays after pentecost and {} after epiphany giving {} sundays after epiphany to be resumed", x, sundays_after_pentecost, sundays_after_epiphany, sundays_after_pentecost - 24);
        }
    }
}
