use chrono::{NaiveDate, Datelike, Weekday};
use crate::liturgy::{Kalendar, Celebration, Penance, Color, Season, Rank, Identifier};
use crate::timehelp::{Betwixt, FullName, Ordinal};

pub fn get_celebration(ly: &Kalendar, date: NaiveDate) -> Celebration {
	use Weekday::*;

	let week_num = (NaiveDate::weeks_since(ly.easter, date) + 1) as u8;

	let identifier = Identifier {
		season: Season::Easter,
		week: week_num.to_string(),
		day: String::from(date.weekday().fullname())
	};

	let (name, rank) = match date.weekday() {
		Sun => (
			format!("{} Sunday after the Pasch", (week_num - 1).ordinal()),
			Rank::Sunday
			),
		_ => (
			format!("{} in the {} Week after the Pasch", date.weekday().fullname(), (week_num - 1).ordinal()),
			Rank::Feria
			)
	};

	let penance = match date.weekday() {
		Wed => Some(Penance::Abstinence),
		Fri => Some(Penance::Abstinence),
		_ => None
	};

	let color = Color::White;

	match week_num {
		1 => match date.weekday() {
			Sun => Celebration {
				name: String::from("Great and Holy Pasch of the Lord"),
				penance: None,
				rank: Rank::Triplex,
				color,
				identifier
			},

			_ => Celebration {
				name: format!("{} in the Paschal Octave", date.weekday().fullname()),
				penance: None,
				rank: Rank::StrongFeria,
				color,
				identifier
			}
		},

		2 | 3 | 4 | 5 => Celebration {
			name, penance, rank, color, identifier
		},

		6 => match date.weekday() {
			Sun | Mon | Tue => Celebration {
				name, penance, rank, color, identifier
			},
			_ => ascension(ly, date, week_num, identifier)
		},

		7 => ascension(ly, date, week_num, identifier),

		8 => pentecost(ly, date, identifier),

		_ => panic!("Requested easter week {}; only 7 exist", week_num)
	}
}

fn pentecost(ly: &Kalendar, date: NaiveDate, identifier: Identifier) -> Celebration {
	use Weekday::*;
	let color = Color::Red;
	match date.weekday() {
		Sun => Celebration {
			name: String::from("Sunday of the Pentecost"),
			color,
			rank: Rank::Triplex,
			penance: None,
			identifier
		},

		Mon | Tue | Thu => Celebration {
			name: format!("{} Day of the Pentecost", (NaiveDate::days_since(ly.pentecost, date) + 1).ordinal()),
			color,
			rank: Rank::Feria,
			penance: None,
			identifier
		},

		Wed | Fri => Celebration {
			name: format!("Ember {} of the Pentecost", date.weekday().fullname()),
			color,
			rank: Rank::Feria,
			penance: Some(Penance::Fasting),
			identifier
		},

		Sat => Celebration {
			name: String::from("Ember Saturday of the Pentecost"),
			color,
			rank: Rank::Feria,
			penance: Some(Penance::Vigil),
			identifier
		}
	}
}

fn ascension(ly: &Kalendar, date: NaiveDate, week_num: u8, identifier: Identifier) -> Celebration {
	use Weekday::*;
	match week_num {
		6 => {
			match date.weekday() {
				Wed => Celebration {
					name: String::from("Eve of the Ascension"),
					penance: Some(Penance::Vigil),
					color: Color::Violet,
					rank: Rank::Feria,
					identifier
				},

				Thu => Celebration {
					name: String::from("Ascension of the Lord"),
					penance: None,
					color: Color::White,
					rank: Rank::Triplex,
					identifier
				},

				Fri | Sat => Celebration {
					name: format!("{} Day of the Ascension", (NaiveDate::days_since(ly.ascension, date) + 2).ordinal()),
					penance: None,
					rank: Rank::Feria,
					color: Color::White,
					identifier
				},

				_ => panic!("Requested ascension octave for {} in first week", date.weekday())
			}
		},
		
		7 => {
			match date.weekday() {
				Sun => Celebration {
					name: String::from("Sunday after the Ascension"),
					color: Color::White,
					penance: None,
					rank: Rank::StrongSunday,
					identifier
				},

				Mon | Tue | Wed => Celebration {
					name: format!("{} Day of the Ascension", (NaiveDate::days_since(ly.ascension, date) + 2).ordinal()),
					color: Color::White,
					penance: None,
					rank: Rank::Feria,
					identifier
				},

				Thu => Celebration {
					name: String::from("Octave of the Ascension"),
					color: Color::White,
					penance: None,
					rank: Rank::Duplex,
					identifier
				},

				Fri => Celebration {
					name: String::from("Friday in Ascensiontide"),
					color: Color::White,
					penance: Some(Penance::Abstinence),
					rank: Rank::Feria,
					identifier
				},

				Sat => Celebration {
					name: String::from("Eve of the Pentecost"),
					color: Color::White,
					penance: Some(Penance::Vigil),
					rank: Rank::Feria,
					identifier
				}
			}
		},

		_ => panic!("Requested ascension for easter week {}", week_num)
	}
}