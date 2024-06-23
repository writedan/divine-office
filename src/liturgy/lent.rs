use chrono::{NaiveDate, Datelike, Weekday};
use crate::liturgy::{Kalendar, Celebration, Penance, Color, Season, Rank, Identifier};
use crate::timehelp::{Betwixt, FullName, Ordinal, Sunday};

pub fn get_celebration(ly: &Kalendar, date: NaiveDate) -> Celebration {
	use Weekday::*;

	let week_num = (NaiveDate::weeks_since(ly.ash_wednesday.prev_sunday().unwrap(), date)) as u8;

	let identifier = vec![Identifier {
		season: Season::Lent,
		week: week_num.to_string(),
		day: String::from(date.weekday().fullname())
	}];

	match week_num {
		0 => quinquagesima(ly, date, identifier),
		1 => lent1(ly, date, identifier, week_num),
		2..=4 => lent(ly, date, identifier, week_num),
		5 => passion(ly, date, identifier),
		6 => holyweek(ly, date, identifier),
		_ => panic!("There are only 6 weeks of Lent, requested week {}", week_num)
	}
}

fn holyweek(ly: &Kalendar, date: NaiveDate, identifier: Vec<Identifier>) -> Celebration {
	use Weekday::*;
	let (color, penance, rank) = (Color::Violet, Some(Penance::Fasting), Rank::StrongFeria);
	match date.weekday() {
		Sun => Celebration {
			name: String::from("Palm Sunday of the Passion"),
			color,
			penance: Some(Penance::Abstinence),
			rank: Rank::StrongSunday,
			identifier
		},

		Mon | Tue | Wed => Celebration {
			name: format!("{} in Holy Week", date.weekday().fullname()),
			color,
			penance,
			rank,
			identifier
		},

		Thu => Celebration {
			name: String::from("Thursday of the Lord's Supper"),
			color,
			penance: Some(Penance::Vigil),
			rank,
			identifier
		},

		Fri => Celebration {
			name: String::from("Friday of the Preparation"),
			color: Color::Black,
			penance: Some(Penance::Vigil),
			rank,
			identifier
		},

		Sat => Celebration {
			name: String::from("Holy Saturday of the Paschal Vigil"),
			color,
			penance: Some(Penance::Vigil),
			rank,
			identifier
		}
	}
}

fn passion(ly: &Kalendar, date: NaiveDate, identifier: Vec<Identifier>) -> Celebration {
	use Weekday::*;
	let (color, penance, rank) = (Color::Violet, Some(Penance::Fasting), Rank::Feria);
	match date.weekday() {
		Sun => Celebration {
			name: String::from("Sunday before the Passion"),
			color,
			penance: Some(Penance::Abstinence),
			rank: Rank::StrongSunday,
			identifier
		},

		_ => Celebration {
			name: format!("{} in the Week before the Passion", date.weekday().fullname()),
			color,
			penance,
			rank,
			identifier
		}
	}
}

fn lent(ly: &Kalendar, date: NaiveDate, identifier: Vec<Identifier>, week_num: u8) -> Celebration {
	use Weekday::*;
	let (color, penance, rank) = (Color::Violet, Some(Penance::Fasting), Rank::Feria);
	match date.weekday() {
		Sun => Celebration {
			name: format!("{} Sunday in Lent", week_num.ordinal()),
			rank,
			penance: Some(Penance::Abstinence),
			color,
			identifier
		},

		_ => Celebration {
			name: format!("{} in the {} Week of Lent", date.weekday().fullname(), week_num.ordinal()),
			rank,
			penance,
			color,
			identifier
		}
	}
}

fn lent1(ly: &Kalendar, date: NaiveDate, identifier: Vec<Identifier>, week_num: u8) -> Celebration {
	use Weekday::*;
	let color = Color::Violet;
	let penance = Some(Penance::Fasting);
	let rank = Rank::Feria;
	match date.weekday() {
		Sun => Celebration {
			name: format!("{} Sunday in Lent", week_num.ordinal()),
			color,
			identifier,
			rank: Rank::StrongSunday,
			penance: Some(Penance::Abstinence)
		},

		Wed => Celebration {
			name: String::from("Ember Wednesday of Lent"),
			color,
			identifier,
			penance,
			rank
		},

		Fri => Celebration {
			name: String::from("Ember Friday of Lent"),
			color,
			identifier,
			penance,
			rank
		},

		Sat => Celebration {
			name: String::from("Ember Saturday of Lent"),
			color,
			identifier,
			penance: Some(Penance::Vigil),
			rank
		},

		_ => Celebration {
			name: format!("{} in the {} Week of Lent", date.weekday().fullname(), week_num.ordinal()),
			color,
			identifier,
			rank,
			penance
		}
	}
}

fn quinquagesima(ly: &Kalendar, date: NaiveDate, identifier: Vec<Identifier>) -> Celebration {
	use Weekday::*;
	let (name, penance, color, rank) = (format!("{} after the Ashes", date.weekday().fullname()), Some(Penance::Fasting), Color::Violet, Rank::Feria);

	match date.weekday() {
		Wed => Celebration {
			name: String::from("Ash Wednesday"),
			penance,
			color,
			rank: Rank::StrongFeria,
			identifier
		},

		_ => Celebration {
			name, penance, color, rank, identifier
		}
	}
}