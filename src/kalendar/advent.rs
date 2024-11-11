use chrono::{NaiveDate, Datelike, Weekday};
use crate::kalendar::{Kalendar, Celebration, Penance, Color, Season, Rank, Identifier};
use crate::timehelp::{Betwixt, FullName, Ordinal};

pub fn get_celebration(ly: &Kalendar, date: NaiveDate) -> Celebration {
	let week_num = (NaiveDate::weeks_since(ly.advent, date) + 1) as u8;

	let (name, color, penance, rank) = match date.weekday() {
		Weekday::Sun => (
			format!("{} Sunday of Advent", week_num.ordinal()),
			if week_num == 3 { Color::Rose } else { Color::Violet },
			None,
			Rank::StrongSunday
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

	let mut identifiers = Vec::<Identifier>::new();
	identifiers.push(Identifier {
		season: Season::Advent,
		week: week_num.to_string(),
		day: String::from(date.weekday().fullname())
	});

	let o_wisdom = NaiveDate::from_ymd_opt(date.year(), 12, 17).unwrap();

	if date >= o_wisdom {
		identifiers.push(Identifier {
			season: Season::Advent,
			week: String::from("o-antiphons"),
			day: (NaiveDate::days_since(o_wisdom, date) + 1).to_string()
		});
	}

	Celebration {
		name,
		color,
		penance,
		rank,
		identifiers
	}
}