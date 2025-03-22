mod advent;
mod christmas;
mod easter;
mod lent;
mod postepiphany;
mod postpentecost;
mod prelent;

use crate::timehelp::{Betwixt, Sunday};
use chrono::{Datelike, Days, NaiveDate};

use std::cmp::Ordering;

#[derive(Debug)]
pub struct Kalendar {
    // these are the sentiels of the calendar
    advent: NaiveDate,
    christmas: NaiveDate,
    epiphany_sunday: NaiveDate, // the sunday after epiphany
    purification: NaiveDate,
    septuagesima: NaiveDate,
    ash_wednesday: NaiveDate,
    easter: NaiveDate,
    ascension: NaiveDate,
    pentecost: NaiveDate,
    next_advent: NaiveDate, // for validation purposes only
}

#[derive(Eq, PartialEq, Hash, Debug, serde::Serialize, Clone)]
pub enum Penance {
    Abstinence,
    Fasting,
    Vigil,
}

#[derive(Eq, PartialEq, Hash, Debug, serde::Serialize, Clone)]
pub enum Color {
    White,
    Blue,
    Green,
    Red,
    Black,
    Violet,
    Rose,
}

#[derive(Eq, PartialEq, Hash, Debug, PartialOrd, serde::Serialize, Clone)]
pub enum Rank {
    Eve,
    Feria,
    StrongFeria, // cannot be superseded by anything
    Simplex,
    Semiduplex,
    Sunday,
    Duplex,
    StrongSunday,
    Triplex,
}

#[derive(Eq, PartialEq, Hash, Debug, serde::Serialize, Clone)]
pub enum Season {
    Advent,
    AdventSpecial, // to support O Antiphons
    Christmas,
    PostEpiphany(bool), // whether we are before (true) or after (false) the Purification
    PreLent(bool),      // see note above
    Lent,
    Easter,
    PostPentecost,
    August,
    September,
    October,
    November,
}

impl ToString for Season {
    fn to_string(&self) -> String {
        match self {
            crate::kalendar::Season::Advent => String::from("advent"),
            crate::kalendar::Season::AdventSpecial => String::from("advent"),
            crate::kalendar::Season::Christmas => String::from("christmas"),
            crate::kalendar::Season::PostEpiphany(_) => String::from("post-epiphany"),
            crate::kalendar::Season::PreLent(_) => String::from("pre-lent"),
            crate::kalendar::Season::Lent => String::from("lent"),
            crate::kalendar::Season::Easter => String::from("easter"),
            crate::kalendar::Season::PostPentecost => String::from("post-pentecost"),
            crate::kalendar::Season::August => String::from("august"),
            crate::kalendar::Season::September => String::from("september"),
            crate::kalendar::Season::October => String::from("october"),
            crate::kalendar::Season::November => String::from("november"),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, serde::Serialize, Clone)]
pub struct Identifier {
    pub season: Season,
    pub week: String,
    pub day: String,
}

#[derive(Eq, PartialEq, Hash, Debug, serde::Serialize, Clone)]
pub struct Celebration {
    pub name: String,
    pub penance: Option<Penance>,
    pub color: Color,
    pub rank: Rank,
    identifiers: Vec<Identifier>,
}

impl Ord for Celebration {
    fn cmp(&self, other: &Self) -> Ordering {
        let r1 = &self.rank;
        let r2 = &other.rank;

        if *r1 > *r2 {
            Ordering::Greater
        } else if *r2 > *r1 {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Celebration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}

impl Kalendar {
    pub fn from_date(date: NaiveDate) -> Option<Kalendar> {
        let advent = NaiveDate::from_ymd_opt(date.year(), 11, 27)?.this_or_next_sunday()?;
        if date < advent {
            Kalendar::from_year(date.year() - 1)
        } else {
            Kalendar::from_year(date.year())
        }
    }

    // option is returned since theoretically a user could pass in a bad year
    // we don't want to bring the server crashing down because of it
    pub fn from_year(year: i32) -> Option<Kalendar> {
        let easter = computus::gregorian_naive(year + 1).ok()?; // liturgical year begins advent year prior
        Some(Kalendar {
            advent: NaiveDate::from_ymd_opt(year, 11, 27)?.this_or_next_sunday()?,
            christmas: NaiveDate::from_ymd_opt(year, 12, 24)?,
            epiphany_sunday: NaiveDate::from_ymd_opt(year + 1, 1, 6)?.next_sunday()?,
            purification: NaiveDate::from_ymd_opt(year + 1, 2, 2)?,
            septuagesima: easter.checked_sub_days(Days::new(63))?,
            ash_wednesday: easter.checked_sub_days(Days::new(46))?,
            easter,
            ascension: easter.checked_add_days(Days::new(40))?,
            pentecost: easter.checked_add_days(Days::new(49))?,
            next_advent: NaiveDate::from_ymd_opt(year + 1, 11, 27)?.this_or_next_sunday()?,
        })
    }

    fn get_season(&self, date: NaiveDate) -> Season {
        let first_of = if self.purification < self.septuagesima {
            self.purification
        } else {
            self.septuagesima
        };

        let seasons = [
            (Season::Advent, self.advent, self.christmas),
            (Season::Christmas, self.christmas, self.epiphany_sunday),
            (Season::PostEpiphany(true), self.epiphany_sunday, first_of),
            (Season::PreLent(true), self.septuagesima, self.purification),
            (
                Season::PreLent(false),
                self.septuagesima,
                self.ash_wednesday,
            ),
            (
                Season::PostEpiphany(false),
                self.purification,
                self.septuagesima,
            ),
            (Season::Lent, self.ash_wednesday, self.easter),
            (
                Season::Easter,
                self.easter,
                self.pentecost.next_sunday().unwrap(),
            ), // this case can be safely unwrapped since we have a valid kalendar
            (
                Season::PostPentecost,
                self.pentecost.next_sunday().unwrap(),
                self.next_advent,
            ),
        ];

        for (season, start, end) in seasons {
            if date.is_between(start, end) {
                return season;
            }
        }

        panic!(
            "Requested season of a date beyond the bounds of liturgical year {}.",
            self.advent.year()
        );
    }

    fn get_temporal(&self, date: NaiveDate) -> Celebration {
        match self.get_season(date) {
            Season::Advent => advent::get_celebration(self, date),
            Season::Christmas => christmas::get_celebration(self, date),
            Season::PostEpiphany(_) => postepiphany::get_celebration(self, date),
            Season::PreLent(_) => prelent::get_celebration(self, date),
            Season::Lent => lent::get_celebration(self, date),
            Season::Easter => easter::get_celebration(self, date),
            Season::PostPentecost => postpentecost::get_celebration(self, date),
            _ => panic!(
                "{:?} should not be returned from Kalendar.get_season.",
                self.get_season(date)
            ),
        }
    }

    pub fn get_celebrations(&self, date: NaiveDate) -> Vec<Celebration> {
        let mut vec = vec![self.get_temporal(date)];
        vec.sort();
        vec
    }
}

pub fn get_celebration(date: NaiveDate) -> Option<Celebration> {
    let ly = Kalendar::from_date(date)?;
    Some(ly.get_celebrations(date)[0].clone())
}

impl Celebration {
    pub fn identifiers(&self) -> Vec<&Identifier> {
        self.identifiers.iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_computus() {
        let ly = Kalendar::from_year(2023).unwrap();
        assert_eq!(ly.advent, NaiveDate::from_ymd(2023, 12, 3));
        assert_eq!(ly.epiphany_sunday, NaiveDate::from_ymd(2024, 1, 7));
        assert_eq!(ly.septuagesima, NaiveDate::from_ymd(2024, 1, 28));
        assert_eq!(ly.easter, NaiveDate::from_ymd(2024, 3, 31));
        assert_eq!(ly.pentecost, NaiveDate::from_ymd(2024, 5, 19));

        let ly = Kalendar::from_year(1817).unwrap();
        assert_eq!(ly.advent, NaiveDate::from_ymd(1817, 11, 30));
        assert_eq!(ly.epiphany_sunday, NaiveDate::from_ymd(1818, 1, 11));
        assert_eq!(ly.septuagesima, NaiveDate::from_ymd(1818, 1, 18));
        assert_eq!(ly.easter, NaiveDate::from_ymd(1818, 3, 22));
        assert_eq!(ly.pentecost, NaiveDate::from_ymd(1818, 5, 10));

        let ly = Kalendar::from_year(2004).unwrap();
        assert_eq!(ly.advent, NaiveDate::from_ymd(2004, 11, 28));
        assert_eq!(ly.epiphany_sunday, NaiveDate::from_ymd(2005, 1, 9));
        assert_eq!(ly.septuagesima, NaiveDate::from_ymd(2005, 1, 23));
        assert_eq!(ly.easter, NaiveDate::from_ymd(2005, 3, 27));
        assert_eq!(ly.pentecost, NaiveDate::from_ymd(2005, 5, 15));

        let ly = Kalendar::from_year(2024).unwrap();
        assert_eq!(ly.advent, NaiveDate::from_ymd(2024, 12, 1));
        assert_eq!(ly.epiphany_sunday, NaiveDate::from_ymd(2025, 1, 12));
        assert_eq!(ly.septuagesima, NaiveDate::from_ymd(2025, 2, 16));
        assert_eq!(ly.easter, NaiveDate::from_ymd(2025, 4, 20));
        assert_eq!(ly.pentecost, NaiveDate::from_ymd(2025, 6, 8));

        let ly = Kalendar::from_year(2025).unwrap();
        assert_eq!(ly.advent, NaiveDate::from_ymd(2025, 11, 30));
        assert_eq!(ly.epiphany_sunday, NaiveDate::from_ymd(2026, 1, 11));
        assert_eq!(ly.septuagesima, NaiveDate::from_ymd(2026, 2, 1));
        assert_eq!(ly.easter, NaiveDate::from_ymd(2026, 4, 5));
        assert_eq!(ly.pentecost, NaiveDate::from_ymd(2026, 5, 24));
    }

    #[test]
    fn test_seasons() {
        let ly = Kalendar::from_year(2024).unwrap();
        let d = NaiveDate::from_ymd(2024, 12, 1);
        assert_eq!(ly.get_season(d), Season::Advent);

        let d = NaiveDate::from_ymd(2024, 12, 28);
        assert_eq!(ly.get_season(d), Season::Christmas);

        let d = NaiveDate::from_ymd(2025, 8, 9);
        assert_eq!(ly.get_season(d), Season::PostPentecost);

        let d = NaiveDate::from_ymd(2024, 12, 23);
        assert_eq!(ly.get_season(d), Season::Advent);
    }
}
