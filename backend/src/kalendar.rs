mod advent;
mod christmas;
mod postepiphany;
mod prelent;
mod lent;
mod easter;
mod postpentecost;

use chrono::{NaiveDate, Weekday, Datelike};
use crate::timehelp::{Sunday, Betwixt};
use std::cmp::Ordering;

/// The kalendar provides the liturgical identifiers for a given Gregorian date. The fields of its struct are used as sentiels to get the correct identifier. A kalendar is created from a year which indicates the liturgical year, not the calendar year. Thus a kalendar runs from November or December of its specified year to that of the following year.
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
	next_advent: NaiveDate
}

/// Penance describes both the fasting rules of the day and when Mass may be said that day.
#[derive(Eq, PartialEq)]
pub enum Penance {
	/// Abstinence means refraining from meat, dairy, and eggs. Mass is after Terce.
	Abstinence, 

	/// Fasting means refraining from meat, fish, oil, wine, dairy, and eggs. Mass is after Sext.
	Fasting,

	/// Vigil has the same exclusions as Fasting but Mass is said after None.
	Vigil
}

#[derive(Eq, PartialEq)]
pub enum Color {
    White,
    Blue,
    Green,
    Red,
    Black,
    Violet,
    Rose,
}

#[derive(PartialEq, PartialOrd, Eq)]
pub enum Rank {
    Eve,
    Feria,

    /// A strong feria cannot be superseceded by any other celebration.
    StrongFeria,
    Simplex,
    Semiduplex,
    Sunday,
    Duplex,
    StrongSunday,
    Triplex,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
#[derive(Eq, PartialEq)]
pub struct Identifier {
    pub season: Season,
    /// The week of the season this office occurs in. "Week of the season" is particular to each season.
    pub week: String,
    /// The day of the seasonal week this office occurs in. This meaning is particular to each season.
    pub day: String,
    /// The actual day of the calendar week this office occurs on.
    pub weekday: Weekday
}

/// A celebration is the actual liturgical day. The liturgical day has one name, one penance, one color, and one rank, but may be composed out of multiple identifiers. In this case the first identifier has the right of way and subsequent identifiers can only add what is not defined in the first identifier.
#[derive(Eq, PartialEq)]
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

impl ToString for Season {
    fn to_string(&self) -> String {
    	use crate::kalendar::Season::*;
        match self {
            Advent => String::from("advent"),
            Christmas => String::from("christmas"),
            PostEpiphany(_) => String::from("post-epiphany"),
            PreLent(_) => String::from("pre-lent"),
            Lent => String::from("lent"),
            Easter => String::from("easter"),
            PostPentecost => String::from("post-pentecost"),
            August => String::from("august"),
            September => String::from("september"),
            October => String::from("october"),
            November => String::from("november"),
        }
    }
}

impl Kalendar {
	pub fn from_date(date :NaiveDate) -> Option<Kalendar> {
		let advent = NaiveDate::from_ymd_opt(date.year(), 11, 27)?.this_or_next_sunday()?; // advent falls on the first sunday on or after the 27th of November.

		if date < advent {
			// if the specified date falls before advent, we need the kalendar for the year prior
            Kalendar::from_year(date.year() - 1)
        } else {
            Kalendar::from_year(date.year())
        }
	}

	/// Generates a liturgical calendar for the given liturgical year.
	pub fn from_year(year: i32) -> Option<Kalendar> {
		use chrono::Days;

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
                self.pentecost.next_sunday().unwrap(), // this case can be safely unwrapped since we have a valid kalendar
            ),
            (
                Season::PostPentecost,
                self.pentecost.next_sunday().unwrap(),
                self.next_advent,
            ),
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

	    // This panic should be unreachable unless there is something wrong with our internal code.
	}

	/// Returns the temporal celebration for the given date. There can only be one temporal celebration for any given date.
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

    /// Returns all celebrations for the given date and sorts them in order of rank.
    pub fn get_celebrations(&self, date: NaiveDate) -> Vec<Celebration> {
        let mut vec = vec![self.get_temporal(date)];
        vec.sort();
        vec
    }
}