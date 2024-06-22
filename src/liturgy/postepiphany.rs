use chrono::{NaiveDate, Datelike, Weekday};
use crate::liturgy::{Kalendar, Celebration, Penance, Color, Season, Rank, Identifier};
use crate::timehelp::{Betwixt, FullName, Ordinal};

pub fn get_celebration(ly: &Kalendar, date: NaiveDate) -> Celebration {
	use Weekday::*;

	let week_num = (NaiveDate::weeks_since(ly.epiphany_sunday, date) + 1) as u8;

	let identifier = Identifier {
		season: Season::PostEpiphany,
		week: week_num.to_string(),
		day: String::from(date.weekday().fullname())
	};

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
		identifier
	}
}