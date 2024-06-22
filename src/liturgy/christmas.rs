use chrono::{NaiveDate, Datelike, Weekday};
use crate::liturgy::{Kalendar, Celebration, Penance, Color, Season, Rank, Identifier};
use crate::timehelp::{Betwixt, FullName};

pub fn get_celebration(ly: &Kalendar, date: NaiveDate) -> Celebration {
	let distance = NaiveDate::days_since(ly.christmas, date);
	let identifier = Identifier {
		season: Season::Christmas,
		week: String::from("0"),
		day: distance.to_string()
	};

	match distance {
		0 => return Celebration {
			name: if date.weekday() == Weekday::Sun {String::from("Eve of the Nativity on Sunday")} else {String::from("Eve of the Nativity")},
			penance: if date.weekday() == Weekday::Sun {None} else {Some(Penance::Vigil)},
			color: Color::Violet,
			rank: Rank::StrongFeria,
			identifier
		},

		1 => return Celebration {
			name: String::from("Nativity of the Lord"),
			penance: None,
			color: Color::White,
			rank: Rank::Triplex,
			identifier
		},

		2 => return Celebration {
			name: String::from("Saint Stephen, Protomartyr"),
			penance: None,
			color: Color::Red,
			rank: Rank::Duplex,
			identifier
		},

		3 => return Celebration {
			name: String::from("Saint John, Apostle and Evangelist"),
			penance: None,
			color: Color::White,
			rank: Rank::Duplex,
			identifier
		},

		4 => return Celebration {
			name: String::from("Holy Innocents, Martyrs"),
			penance: None,
			color: Color::Violet,
			rank: Rank::Duplex,
			identifier
		},

		5 => return Celebration {
			name: String::from("Saint Thomas Becket, Bishop and Martyr"),
			penance: None,
			color: Color::Red,
			rank: Rank::Duplex,
			identifier
		},

		6 => return Celebration {
			name: String::from("Sixth Day of the Nativity"),
			penance: None,
			color: Color::White,
			rank: Rank::StrongFeria,
			identifier
		},

		7 => return Celebration {
			name: String::from("Seventh Day of the Nativity"),
			penance: None,
			color: Color::White,
			rank: Rank::StrongFeria,
			identifier
		},

		8 => return Celebration {
			name: String::from("Circumcision of the Lord"),
			penance: None,
			color: Color::White,
			rank: Rank::Triplex,
			identifier
		},

		9 => return Celebration {
			name: String::from("Octave of Saint Stephen"),
			penance: None,
			color: Color::Red,
			rank: Rank::StrongFeria,
			identifier
		},

		10 => return Celebration {
			name: String::from("Octave of Saint John"),
			penance: None,
			color: Color::White,
			rank: Rank::StrongFeria,
			identifier
		},

		11 => return Celebration {
			name: String::from("Octave of the Innocents"),
			penance: None,
			color: Color::Red,
			rank: Rank::StrongFeria,
			identifier
		},

		12 => return Celebration {
			name: if date.weekday() == Weekday::Sun { String::from("Eve of the Epiphany on Sunday") } else { String::from("Eve of the Epiphany") },
			penance: if date.weekday() == Weekday::Sun { None } else { Some(Penance::Vigil) },
			color: Color::Violet,
			rank: Rank::StrongFeria,
			identifier
		},

		13 => return Celebration {
			name: String::from("Epiphany of the Lord"),
			penance: None,
			color: Color::White,
			rank: Rank::Triplex,
			identifier
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
		identifier
	}
}