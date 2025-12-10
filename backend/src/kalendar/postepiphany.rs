use crate::kalendar::{Celebration, Color, Identifier, Kalendar, Penance, Rank, Season};
use crate::timehelp::{Betwixt, FullName, Ordinal};
use chrono::{Datelike, NaiveDate, Weekday};

pub fn get_celebration(ly: &Kalendar, date: NaiveDate) -> Celebration {
    use Weekday::*;
    let week_num = (NaiveDate::weeks_since(ly.epiphany_sunday, date) + 1) as u8;
    let weekday = date.weekday();
    let weekday_name = weekday.fullname();

    let identifiers = vec![Identifier {
        season: Season::PostEpiphany(date <= ly.purification),
        week: week_num.to_string(),
        day: String::from(weekday_name),
        weekday,
    }];

    let (name, rank) = match weekday {
        Sun => (
            format!("{} Sunday after Epiphany", week_num.ordinal()),
            Rank::Sunday,
        ),
        _ => (
            format!(
                "{} in the {} Week after Epiphany",
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

    Celebration {
        name,
        penance,
        color: Color::Green,
        rank,
        identifiers,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epiphany_sundays() {
        for x in 1600..=3000 {
            let ly = Kalendar::from_year(x).unwrap();
            let num_sundays = NaiveDate::weeks_since(ly.epiphany_sunday, ly.septuagesima);
            assert!(
                num_sundays <= 6,
                "there are {} sundays after epiphany in year {}",
                num_sundays,
                x
            );
        }
    }
}
