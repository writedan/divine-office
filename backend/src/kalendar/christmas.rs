use crate::kalendar::{Celebration, Color, Identifier, Kalendar, Penance, Rank, Season};
use crate::timehelp::{Betwixt, FullName};
use chrono::{Datelike, NaiveDate, Weekday};

pub fn get_celebration(ly: &Kalendar, date: NaiveDate) -> Celebration {
    let distance = NaiveDate::days_since(ly.christmas, date);
    let weekday = date.weekday();
    let is_sunday = weekday == Weekday::Sun;

    let identifiers = vec![Identifier {
        season: Season::Christmas,
        week: match distance {
            0 => String::from("christmas-eve"),
            1 => String::from("christmas-day"),
            2..=7 => String::from("christmas-octave"),
            8 => String::from("circumcision"),
            9..=11 => String::from("post-octave"),
            12..=13 => String::from("epiphany"),
            _ => String::from("christmastide"),
        },
        day: match distance {
            0 => weekday.fullname().to_string(),
            1 => String::from(""),
            2..=7 => format!("{}", distance - 1),
            8 => String::from(""),
            9..=11 => format!("{}", distance - 8),
            12 => String::from("eve"),
            13 => String::from("day"),
            _ => weekday.fullname().to_string(),
        },
        weekday,
    }];

    let (name, color, penance, rank) = match distance {
        0 => (
            if is_sunday {
                "Vigil of the Nativity on Sunday"
            } else {
                "Vigil of the Nativity"
            }
            .to_string(),
            Color::Violet,
            if is_sunday {
                None
            } else {
                Some(Penance::Vigil)
            },
            Rank::Vigil,
        ),
        1 => (
            "Nativity of the Lord".to_string(),
            Color::White,
            None,
            Rank::Triplex,
        ),
        2 => (
            "Saint Stephen, Protomartyr".to_string(),
            Color::Red,
            None,
            Rank::Duplex,
        ),
        3 => (
            "Saint John, Apostle and Evangelist".to_string(),
            Color::White,
            None,
            Rank::Duplex,
        ),
        4 => (
            "Holy Innocents, Martyrs".to_string(),
            Color::Violet,
            None,
            Rank::Duplex,
        ),
        5 => (
            "Saint Thomas Becket, Bishop and Martyr".to_string(),
            Color::Red,
            None,
            Rank::Duplex,
        ),
        6 => (
            "Sixth Day of the Nativity".to_string(),
            Color::White,
            None,
            Rank::Feria,
        ),
        7 => (
            "Seventh Day of the Nativity".to_string(),
            Color::White,
            None,
            Rank::Feria,
        ),
        8 => (
            "Circumcision of the Lord".to_string(),
            Color::White,
            None,
            Rank::Triplex,
        ),
        9 => (
            "Octave of Saint Stephen".to_string(),
            Color::Red,
            None,
            Rank::Feria,
        ),
        10 => (
            "Octave of Saint John".to_string(),
            Color::White,
            None,
            Rank::Feria,
        ),
        11 => (
            "Octave of the Innocents".to_string(),
            Color::Red,
            None,
            Rank::Feria,
        ),
        12 => (
            if is_sunday {
                "Vigil of the Epiphany on Sunday"
            } else {
                "Vigil of the Epiphany"
            }
            .to_string(),
            Color::Violet,
            if is_sunday {
                None
            } else {
                Some(Penance::Vigil)
            },
            Rank::Vigil,
        ),
        13 => (
            "Epiphany of the Lord".to_string(),
            Color::White,
            None,
            Rank::Triplex,
        ),
        _ => (
            format!("{} in Christmastide", weekday.fullname()),
            Color::White,
            match weekday {
                Weekday::Wed | Weekday::Fri => Some(Penance::Abstinence),
                _ => None,
            },
            Rank::Feria,
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
