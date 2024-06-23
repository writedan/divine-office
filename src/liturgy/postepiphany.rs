use chrono::{NaiveDate, Datelike, Weekday};
use crate::liturgy::{Kalendar, Celebration, Penance, Color, Season, Rank, Identifier};
use crate::timehelp::{Betwixt, FullName, Ordinal};

pub fn get_celebration(ly: &Kalendar, date: NaiveDate) -> Celebration {
	use Weekday::*;

	let week_num = (NaiveDate::weeks_since(ly.epiphany_sunday, date) + 1) as u8;

	let identifiers = vec![Identifier {
		season: Season::PostEpiphany,
		week: week_num.to_string(),
		day: String::from(date.weekday().fullname())
	}];

	let (name, rank) = match date.weekday() {
		Sun => (
			format!("{} Sunday after Epiphany", week_num.ordinal()),
			Rank::Sunday
			),
		_ => (
			format!("{} in the {} Week after Epiphany", date.weekday().fullname(), week_num.ordinal()),
			Rank::Feria
			)
	};

	let penance = match date.weekday() {
		Wed => Some(Penance::Abstinence),
		Fri => Some(Penance::Abstinence),
		_ => None
	};

	Celebration {
		name,
		penance,
		color: Color::Green,
		rank,
		identifiers
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
			assert!(num_sundays <= 6, "there are {} sundays after epiphany in year {}", num_sundays, x);
		}
	}
}