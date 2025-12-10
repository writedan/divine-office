use crate::kalendar::{Celebration, Color, Identifier, Kalendar, Penance, Rank, Season};
use crate::timehelp::{Betwixt, FullName, Ordinal};
use chrono::{Datelike, NaiveDate, Weekday};

pub fn get_celebration(ly: &Kalendar, date: NaiveDate) -> Celebration {
    use Weekday::*;

    let week_num = (NaiveDate::weeks_since(ly.easter, date) + 1) as u8;
    let weekday = date.weekday();
    let weekday_name = weekday.fullname();

    let identifiers = vec![Identifier {
        season: Season::Easter,
        week: week_num.to_string(),
        day: String::from(weekday_name),
        weekday,
    }];

    let (name, color, penance, rank) = match (week_num, weekday) {
        // Week 1: Paschal Octave
        (1, Sun) => (
            "Passover of the Lord".to_string(),
            Color::White,
            None,
            Rank::Triplex,
        ),
        (1, _) => (
            format!("{} of Passover", weekday_name),
            Color::White,
            None,
            Rank::Feria,
        ),

        // Weeks 2-5: Regular Easter season
        (2..=5, Sun) => (
            format!("{} Sunday after Passover", (week_num - 1).ordinal()),
            Color::White,
            None,
            Rank::Sunday,
        ),
        (2..=5, Wed | Fri) => (
            format!(
                "{} in the {} Week after Passover",
                weekday_name,
                (week_num - 1).ordinal()
            ),
            Color::White,
            Some(Penance::Abstinence),
            Rank::Feria,
        ),
        (2..=5, _) => (
            format!(
                "{} in the {} Week after Passover",
                weekday_name,
                (week_num - 1).ordinal()
            ),
            Color::White,
            None,
            Rank::Feria,
        ),

        // Week 6: Ascension begins Wednesday
        (6, Sun) => (
            format!("{} Sunday after Passover", (week_num - 1).ordinal()),
            Color::White,
            None,
            Rank::Sunday,
        ),
        (6, Mon | Tue) => (
            format!(
                "{} in the {} Week after Passover",
                weekday_name,
                (week_num - 1).ordinal()
            ),
            Color::White,
            None,
            Rank::Feria,
        ),
        (6, Wed) => (
            "Eve of the Ascension".to_string(),
            Color::Violet,
            Some(Penance::Vigil),
            Rank::Feria,
        ),
        (6, Thu) => (
            "Ascension of the Lord".to_string(),
            Color::White,
            None,
            Rank::Triplex,
        ),
        (6, Fri | Sat) => (
            format!("{} after the Ascension", weekday_name),
            Color::White,
            None,
            Rank::Feria,
        ),

        // Week 7: Ascension Octave
        (7, Sun) => (
            "Sunday after the Ascension".to_string(),
            Color::White,
            None,
            Rank::StrongSunday,
        ),
        (7, Mon | Tue | Wed) => (
            format!("{} after the Ascension", weekday_name),
            Color::White,
            None,
            Rank::Feria,
        ),
        (7, Thu) => (
            "Octave of the Ascension".to_string(),
            Color::White,
            None,
            Rank::Duplex,
        ),
        (7, Fri) => (
            format!(
                "{} in the {} Week after Passover",
                weekday_name,
                (week_num - 1).ordinal()
            ),
            Color::White,
            Some(Penance::Abstinence),
            Rank::Feria,
        ),
        (7, Sat) => (
            "Eve of Pentecost".to_string(),
            Color::White,
            Some(Penance::Vigil),
            Rank::Feria,
        ),

        // Week 8: Pentecost
        (8, Sun) => (
            "Sunday of Pentecost".to_string(),
            Color::Red,
            None,
            Rank::Triplex,
        ),
        (8, Mon | Tue | Thu) => (
            format!("{} of Pentecost", weekday_name),
            Color::Red,
            None,
            Rank::Feria,
        ),
        (8, Wed | Fri) => (
            format!("Ember {} of the Pentecost", weekday_name),
            Color::Red,
            Some(Penance::Fasting),
            Rank::Feria,
        ),
        (8, Sat) => (
            "Ember Saturday of the Pentecost".to_string(),
            Color::Red,
            Some(Penance::Vigil),
            Rank::Eve,
        ),

        _ => panic!("Requested easter week {}; only 8 exist", week_num),
    };

    Celebration {
        name,
        color,
        penance,
        rank,
        identifiers,
    }
}
