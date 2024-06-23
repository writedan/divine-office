use chrono::{NaiveDate, Datelike, Weekday};
use crate::kalendar::{Kalendar, Celebration, Penance, Color, Season, Rank, Identifier};
use crate::timehelp::{Betwixt, FullName};

pub fn get_celebration(ly: &Kalendar, date: NaiveDate) -> Celebration {
	let distance = NaiveDate::days_since(ly.christmas, date);
	let identifiers = vec![Identifier {
		season: Season::Christmas,
		week: String::from("0"),
		day: distance.to_string()
	}];

	match distance {
		0 => return Celebration {
			name: if date.weekday() == Weekday::Sun {String::from("Eve of the Nativity on Sunday")} else {String::from("Eve of the Nativity")},
			penance: if date.weekday() == Weekday::Sun {None} else {Some(Penance::Vigil)},
			color: Color::Violet,
			rank: Rank::StrongFeria,
			identifiers
		},

		1 => return Celebration {
			name: String::from("Nativity of the Lord"),
			penance: None,
			color: Color::White,
			rank: Rank::Triplex,
			identifiers
		},

		2 => return Celebration {
			name: String::from("Saint Stephen, Protomartyr"),
			penance: None,
			color: Color::Red,
			rank: Rank::Duplex,
			identifiers
		},

		3 => return Celebration {
			name: String::from("Saint John, Apostle and Evangelist"),
			penance: None,
			color: Color::White,
			rank: Rank::Duplex,
			identifiers
		},

		4 => return Celebration {
			name: String::from("Holy Innocents, Martyrs"),
			penance: None,
			color: Color::Violet,
			rank: Rank::Duplex,
			identifiers
		},

		5 => return Celebration {
			name: String::from("Saint Thomas Becket, Bishop and Martyr"),
			penance: None,
			color: Color::Red,
			rank: Rank::Duplex,
			identifiers
		},

		6 => return Celebration {
			name: String::from("Sixth Day of the Nativity"),
			penance: None,
			color: Color::White,
			rank: Rank::StrongFeria,
			identifiers
		},

		7 => return Celebration {
			name: String::from("Seventh Day of the Nativity"),
			penance: None,
			color: Color::White,
			rank: Rank::StrongFeria,
			identifiers
		},

		8 => return Celebration {
			name: String::from("Circumcision of the Lord"),
			penance: None,
			color: Color::White,
			rank: Rank::Triplex,
			identifiers
		},

		9 => return Celebration {
			name: String::from("Octave of Saint Stephen"),
			penance: None,
			color: Color::Red,
			rank: Rank::StrongFeria,
			identifiers
		},

		10 => return Celebration {
			name: String::from("Octave of Saint John"),
			penance: None,
			color: Color::White,
			rank: Rank::StrongFeria,
			identifiers
		},

		11 => return Celebration {
			name: String::from("Octave of the Innocents"),
			penance: None,
			color: Color::Red,
			rank: Rank::StrongFeria,
			identifiers
		},

		12 => return Celebration {
			name: if date.weekday() == Weekday::Sun { String::from("Eve of the Epiphany on Sunday") } else { String::from("Eve of the Epiphany") },
			penance: if date.weekday() == Weekday::Sun { None } else { Some(Penance::Vigil) },
			color: Color::Violet,
			rank: Rank::StrongFeria,
			identifiers
		},

		13 => return Celebration {
			name: String::from("Epiphany of the Lord"),
			penance: None,
			color: Color::White,
			rank: Rank::Triplex,
			identifiers
		},

		_ => {}
	}

	Celebration {
		name: format!("{} in Christmastide", date.weekday().fullname()),
		penance: match date.weekday() {
			Weekday::Wed => Some(Penance::Abstinence),
			Weekday::Fri => Some(Penance::Abstinence),
			_ => None
		},
		color: Color::White,
		rank: Rank::Feria,
		identifiers
	}
}