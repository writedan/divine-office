mod advent;
mod christmas;
mod postepiphany;
mod prelent;
mod lent;
mod easter;
mod postpentecost;

use chrono::{Days, Datelike, NaiveDate, Weekday};
use crate::timehelp::{Betwixt, Sunday};
use std::cmp::Ordering;

/// The kalendar provides the liturgical identifiers for a given Gregorian date. The fields of its struct are used as sentinels to get the correct identifier. A kalendar is created from a year which indicates the liturgical year, not the calendar year. Thus a kalendar runs from November or December of its specified year to that of the following year.
pub struct Kalendar {
    /// The first Sunday in Advent.
    advent: NaiveDate,
    /// The 25th of December of the given year.
    christmas: NaiveDate,
    /// The first Sunday after Epiphany which falls in the year following that specified.
    epiphany_sunday: NaiveDate,
    /// The 2nd of February of the following year.
    purification: NaiveDate,
    /// The Sunday 3 weeks before Ash Wednesday.
    septuagesima: NaiveDate,
    /// 46 days before Easter.
    ash_wednesday: NaiveDate,
    easter: NaiveDate,
    /// 40 days after Easter.
    ascension: NaiveDate,
    /// 50 days after Easter.
    pentecost: NaiveDate,
    /// The first Sunday in Advent of the following liturgical year.
    next_advent: NaiveDate,
}

/// Penance describes both the fasting rules of the day and when Mass may be said that day.
#[derive(Debug, Clone, Copy, Eq, PartialEq, serde::Serialize)]
pub enum Penance {
    /// Abstinence means refraining from meat, dairy, and eggs. Mass is after Terce.
    Abstinence,
    /// Fasting means refraining from meat, fish, oil, wine, dairy, and eggs. Mass is after Sext.
    Fasting,
    /// Vigil has the same exclusions as Fasting but Mass is said after None.
    Vigil,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, serde::Serialize)]
pub enum Color {
    White,
    Blue,
    Green,
    Red,
    Black,
    Violet,
    Rose,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, serde::Serialize)]
pub enum Rank {
    Eve,
    Feria,
    /// A strong feria cannot be superseded by any other celebration.
    StrongFeria,
    Simplex,
    Semiduplex,
    Sunday,
    Duplex,
    StrongSunday,
    Triplex,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub enum Season {
    Advent,
    Christmas,
    /// The specified value is true if we are before the Purification or false afterwards. If true, the BVM endings are used for hymns.
    PostEpiphany(bool),
    /// See note for PostEpiphany.
    PreLent(bool),
    Lent,
    Easter,
    PostPentecost,
    August,
    September,
    October,
    November,
}

/// A liturgical identifier supplies all information necessary to generate a particular office and Mass.
#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct Identifier {
    pub season: Season,
    /// The week of the season this office occurs in. "Week of the season" is particular to each season.
    pub week: String,
    /// The day of the seasonal week this office occurs in. This meaning is particular to each season.
    pub day: String,
    /// The actual day of the calendar week this office occurs on.
    pub weekday: Weekday,
}

/// A celebration is the actual liturgical day. The liturgical day has one name, one penance, one color, and one rank, but may be composed out of multiple identifiers. In this case the first identifier has the right of way and subsequent identifiers can only add what is not defined in the first identifier.
#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct Celebration {
    pub name: String,
    pub penance: Option<Penance>,
    pub color: Color,
    pub rank: Rank,
    pub identifiers: Vec<Identifier>,
}

impl Ord for Celebration {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Celebration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Season {
    pub fn as_str(&self) -> &str {
        use Season::*;
        match self {
            Advent => "advent",
            Christmas => "christmas",
            PostEpiphany(_) => "post-epiphany",
            PreLent(_) => "pre-lent",
            Lent => "lent",
            Easter => "easter",
            PostPentecost => "post-pentecost",
            August => "august",
            September => "september",
            October => "october",
            November => "november",
        }
    }
}

impl ToString for Season {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl Kalendar {
    pub fn from_date(date: NaiveDate) -> Option<Kalendar> {
        let advent = NaiveDate::from_ymd_opt(date.year(), 11, 27)?.this_or_next_sunday()?;
        let year = if date < advent {
            date.year() - 1
        } else {
            date.year()
        };
        Self::from_year(year)
    }

    /// Generates a liturgical calendar for the given liturgical year.
    pub fn from_year(year: i32) -> Option<Kalendar> {
        let easter = computus::gregorian_naive(year + 1).ok()?;
        
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

    /// Once a kalendar is instantiated we can query it for the season of a given date.
    fn get_season(&self, date: NaiveDate) -> Season {
        let first_of = self.purification.min(self.septuagesima);
        
        let pentecost_sunday = self.pentecost.next_sunday().unwrap();
        
        let seasons = [
            (Season::Advent, self.advent, self.christmas),
            (Season::Christmas, self.christmas, self.epiphany_sunday),
            (Season::PostEpiphany(true), self.epiphany_sunday, first_of),
            (Season::PreLent(true), self.septuagesima, self.purification),
            (Season::PreLent(false), self.septuagesima, self.ash_wednesday),
            (Season::PostEpiphany(false), self.purification, self.septuagesima),
            (Season::Lent, self.ash_wednesday, self.easter),
            (Season::Easter, self.easter, pentecost_sunday),
            (Season::PostPentecost, pentecost_sunday, self.next_advent),
        ];
        
        seasons
            .iter()
            .find(|(_, start, end)| date.is_between(*start, *end))
            .map(|(season, _, _)| *season)
            .unwrap_or_else(|| {
                panic!(
                    "Requested season of a date beyond the bounds of liturgical year {}.",
                    self.advent.year()
                )
            })
    }

    /// Returns the temporal celebration for the given date. There can only be one temporal celebration for any given date.
    fn get_temporal(&self, date: NaiveDate) -> Result<Celebration, String> {
        use Season::*;
        let celebration = match self.get_season(date) {
            Advent => advent::get_celebration(self, date),
            Christmas => christmas::get_celebration(self, date),
            PostEpiphany(_) => postepiphany::get_celebration(self, date),
            PreLent(_) => prelent::get_celebration(self, date),
            Lent => lent::get_celebration(self, date),
            Easter => easter::get_celebration(self, date),
            PostPentecost => postpentecost::get_celebration(self, date),
            season => return Err(format!("{:?} should not be returned from Kalendar.get_season.", season)),
        };

        Ok(celebration)
    }

    /// Returns all celebrations for the given date and sorts them in order of rank.
    pub fn get_celebrations(&self, date: NaiveDate) -> Result<Vec<Celebration>, String> {
        let mut celebrations = vec![self.get_temporal(date)?];
        celebrations.sort();
        Ok(celebrations)
    }
}