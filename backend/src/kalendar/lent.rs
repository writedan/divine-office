use crate::kalendar::{Celebration, Color, Identifier, Kalendar, Penance, Rank, Season};
use crate::timehelp::{Betwixt, FullName, Ordinal, Sunday};
use chrono::{Datelike, NaiveDate, Weekday};

pub fn get_celebration(ly: &Kalendar, date: NaiveDate) -> Celebration {
    let week_num = (NaiveDate::weeks_since(ly.ash_wednesday.prev_sunday().unwrap(), date)) as u8;
    let weekday = date.weekday();
    let weekday_name = weekday.fullname();

    let identifiers = vec![Identifier {
        season: Season::Lent,
        week: week_num.to_string(),
        day: String::from(weekday_name),
        weekday,
    }];

    use Weekday::*;

    let (name, color, penance, rank) = match (week_num, weekday) {
        // Quinquagesima (week 0)
        (0, Wed) => (
            "Ash Wednesday".to_string(),
            Color::Violet,
            Some(Penance::Fasting),
            Rank::StrongFeria,
        ),
        (0, _) => (
            format!("{} after the Ashes", weekday_name),
            Color::Violet,
            Some(Penance::Fasting),
            Rank::Feria,
        ),

        // Lent Week 1 - Ember Days
        (1, Sun) => (
            format!("{} Sunday in Lent", week_num.ordinal()),
            Color::Violet,
            Some(Penance::Abstinence),
            Rank::StrongSunday,
        ),
        (1, Wed | Fri) => (
            format!("Ember {} of Lent", weekday_name),
            Color::Violet,
            Some(Penance::Fasting),
            Rank::Feria,
        ),
        (1, Sat) => (
            "Ember Saturday of Lent".to_string(),
            Color::Violet,
            Some(Penance::Vigil),
            Rank::Feria,
        ),
        (1, _) => (
            format!(
                "{} in the {} Week of Lent",
                weekday_name,
                week_num.ordinal()
            ),
            Color::Violet,
            Some(Penance::Fasting),
            Rank::Feria,
        ),

        // Lent Weeks 2-4
        (2..=4, Sun) => (
            format!("{} Sunday in Lent", week_num.ordinal()),
            Color::Violet,
            Some(Penance::Abstinence),
            Rank::StrongSunday,
        ),
        (2..=4, _) => (
            format!(
                "{} in the {} Week of Lent",
                weekday_name,
                week_num.ordinal()
            ),
            Color::Violet,
            Some(Penance::Fasting),
            Rank::Feria,
        ),

        // Passion Week (week 5)
        (5, Sun) => (
            "Sunday before the Passion".to_string(),
            Color::Violet,
            Some(Penance::Abstinence),
            Rank::StrongSunday,
        ),
        (5, _) => (
            format!("{} in the Week before the Passion", weekday_name),
            Color::Violet,
            Some(Penance::Fasting),
            Rank::Feria,
        ),

        // Holy Week (week 6)
        (6, Sun) => (
            "Palm Sunday of the Passion".to_string(),
            Color::Violet,
            Some(Penance::Abstinence),
            Rank::StrongSunday,
        ),
        (6, Mon | Tue | Wed) => (
            format!("{} in Holy Week", weekday_name),
            Color::Violet,
            Some(Penance::Fasting),
            Rank::Feria,
        ),
        (6, Thu) => (
            "Thursday of the Lord's Supper".to_string(),
            Color::Violet,
            Some(Penance::Vigil),
            Rank::Feria,
        ),
        (6, Fri) => (
            "Friday of the Preparation".to_string(),
            Color::Black,
            Some(Penance::Vigil),
            Rank::Feria,
        ),
        (6, Sat) => (
            "Vigil of Passover".to_string(),
            Color::Violet,
            Some(Penance::Vigil),
            Rank::Vigil,
        ),

        _ => panic!(
            "There are only 6 weeks of Lent, requested week {}",
            week_num
        ),
    };

    Celebration {
        name,
        color,
        penance,
        rank,
        identifiers,
    }
}
