mod advent;
mod christmas;
mod postepiphany;
mod prelent;
mod lent;
mod easter;
mod postpentecost;

use chrono::{NaiveDate, Datelike, Days};
use crate::timehelp::{Sunday, Betwixt};
use std::collections::{HashSet};

#[derive(Debug)]
pub struct Kalendar {
	// these are the sentiels of the calendar
	advent: NaiveDate,
	christmas: NaiveDate,
	epiphany_sunday: NaiveDate, // the sunday after epiphany
	septuagesima: NaiveDate,
	ash_wednesday: NaiveDate,
	easter: NaiveDate,
	ascension: NaiveDate,
	pentecost: NaiveDate,
	next_advent: NaiveDate // for validation purposes only
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum Penance {
	Abstinence, Fasting, Vigil
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum Color {
	White, Blue, Green, Red, Black, Violet, Rose
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum Rank {
	Feria,
	StrongFeria, // cannot be superseded by anything
	Simplex,
	Semiduplex,
	Sunday,
	Duplex,
	StrongSunday,
	Triplex
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum Season {
	Advent, Christmas, PostEpiphany, PreLent, Lent, Easter, PostPentecost
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Identifier {
	season: Season,
	week: String,
	day: String
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Celebration {
	name: String,
	penance: Option<Penance>,
	color: Color,
	rank: Rank,
	identifier: Identifier,
}

impl Kalendar {
	// option is returned since theoretically a user could pass in a bad year
	// we don't want to bring the server crashing down because of it
	pub fn from_year(year: i32) -> Option<Kalendar> {
		let easter = computus::gregorian_naive(year + 1).ok()?; // liturgical year begins advent year prior
        Some(Kalendar {
            advent: NaiveDate::from_ymd_opt(year, 11, 27)?.this_or_next_sunday()?,
            christmas: NaiveDate::from_ymd_opt(year, 12, 24)?,
            epiphany_sunday: NaiveDate::from_ymd_opt(year + 1, 1, 6)?.this_or_next_sunday()?,
            septuagesima: easter.checked_sub_days(Days::new(63))?,
            ash_wednesday: easter.checked_sub_days(Days::new(46))?,
            easter,
            ascension: easter.checked_add_days(Days::new(40))?,
            pentecost: easter.checked_add_days(Days::new(49))?,
            next_advent: NaiveDate::from_ymd_opt(year + 1, 11, 27)?.this_or_next_sunday()?
        })
    }

    fn get_season(&self, date: NaiveDate) -> Season {
    	let seasons = [
    		(Season::Advent, self.advent, self.christmas),
    		(Season::Christmas, self.christmas, self.epiphany_sunday),
    		(Season::PostEpiphany, self.epiphany_sunday, self.septuagesima),
    		(Season::PreLent, self.septuagesima, self.ash_wednesday),
    		(Season::Lent, self.ash_wednesday, self.easter),
    		(Season::Easter, self.easter, self.pentecost.next_sunday().unwrap()), // this case can be safely unwrapped since we have a valid kalendar
    		(Season::PostPentecost, self.pentecost.next_sunday().unwrap(), self.next_advent)
    	];

    	for (season, start, end) in seasons {
    		if date.is_between(start, end) {
    			return season;
    		}
    	}

    	panic!("Requested season of a date beyond the bounds of liturgical year {}.", self.advent.year());
    }

    pub fn get_temporal(&self, date: NaiveDate) -> Celebration {
    	match self.get_season(date) {
    		Season::Advent => advent::get_celebration(self, date),
    		Season::Christmas =>christmas::get_celebration(self, date),
    		Season::PostEpiphany => postepiphany::get_celebration(self, date),
    		Season::PreLent => prelent::get_celebration(self, date),
    		Season::Lent => lent::get_celebration(self, date),
    		Season::Easter => easter::get_celebration(self, date),
    		Season::PostPentecost => postpentecost::get_celebration(self, date)
    	}
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
	}
}