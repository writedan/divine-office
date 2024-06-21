use chrono::{NaiveDate, Datelike, Weekday};
use crate::liturgy::{Kalendar, Celebration, Penance, Color, Season, Rank, Identifier};
use crate::timehelp::{Betwixt, Ordinal};

pub fn get_celebration(ly: &Kalendar, date: NaiveDate) -> Celebration {
	let week_num = (NaiveDate::weeks_since(ly.advent, date) + 1) as u8;

	let (name, color, penance, rank) = match date.weekday() {
		Weekday::Sun => (
			format!("{} Sunday of Advent", week_num.ordinal()),
			if week_num == 3 { Color::Rose } else { Color::Violet },
			None,
			Rank::Duplex
		),

		Weekday::Mon => (
			format!("Monday in the {} Week of Advent", week_num.ordinal()),
			Color::Violet,
			None,
			Rank::Feria
		),

		Weekday::Tue => (
			format!("Tuesday in the {} Week of Advent", week_num.ordinal()),
			Color::Violet,
			None,
			Rank::Feria
		),

		Weekday::Wed => (
			if week_num == 3 { String::from("Ember Wednesday of Advent") } else { format!("Wednesday in the {} Week of Advent", week_num.ordinal()) },
			Color::Violet,
			Some(Penance::Fasting),
			Rank::Feria
		),

		Weekday::Thu => (
			format!("Thursday in the {} Week of Advent", week_num.ordinal()),
			Color::Violet,
			None,
			Rank::Feria
		),

		Weekday::Fri => (
			if week_num == 3 { String::from("Ember Friday of Advent") } else { format!("Friday in the {} Week of Advent", week_num.ordinal()) },
			Color::Violet,
			Some(Penance::Fasting),
			Rank::Feria
		),

		Weekday::Sat => (
			if week_num == 3 { String::from("Ember Saturday of Advent") } else { format!("Saturday in the {} Week of Advent", week_num.ordinal()) },
			Color::Violet,
			Some(if week_num == 3 { Penance::Vigil } else { Penance::Fasting }),
			Rank::Feria
		)
	};

	Celebration {
		name,
		color,
		penance,
		rank,
		identifier: Identifier {
			season: Season::Advent,
			week: week_num,
			day: date.weekday()
		}
	}
}