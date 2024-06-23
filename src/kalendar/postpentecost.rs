use chrono::{NaiveDate, Datelike, Weekday};
use crate::kalendar::{Kalendar, Celebration, Penance, Color, Season, Rank, Identifier};
use crate::timehelp::{Betwixt, FullName, Ordinal, Sunday};

pub fn get_celebration(ly: &Kalendar, date: NaiveDate) -> Celebration {
	use Weekday::*;

	let sundays_after_pentecost = NaiveDate::weeks_since(ly.pentecost.next_sunday().unwrap(), ly.next_advent);
	
	let sunday_num = NaiveDate::weeks_since(ly.pentecost, date);

	let week_num = if sunday_num == sundays_after_pentecost {
		28
	} else {
		sunday_num
	} as u8;

	let mut identifiers = Vec::<Identifier>::new();

	if week_num >= 24 && week_num < 28 {
		identifiers.push(Identifier {
			season: Season::PostEpiphany,
			week: ((week_num - 24) + 1).to_string(),
			day: String::from(date.weekday().fullname())
		})
	} else {
		identifiers.push(Identifier {
			season: Season::PostPentecost,
			week: week_num.to_string(),
			day: String::from(date.weekday().fullname())
		})
	}

	let aug_sunday = NaiveDate::from_ymd_opt(date.year(), 7, 28).unwrap().next_sunday().unwrap();
	let sep_sunday = NaiveDate::from_ymd_opt(date.year(), 8, 28).unwrap().next_sunday().unwrap();
	let oct_sunday = NaiveDate::from_ymd_opt(date.year(), 9, 27).unwrap().next_sunday().unwrap();
	let nov_sunday = NaiveDate::from_ymd_opt(date.year(), 10, 28).unwrap().next_sunday().unwrap();

	if date < aug_sunday {
		let (name, rank) = match date.weekday() {
			Sun => (
				format!("{} Sunday after Pentecost", week_num.ordinal()),
				Rank::Sunday
				),
			_ => (
				format!("{} in the {} Week after Pentecost", date.weekday().fullname(), week_num.ordinal()),
				Rank::Feria
				)
		};

		let penance = match date.weekday() {
			Wed => Some(Penance::Abstinence),
			Fri => Some(Penance::Abstinence),
			_ => None
		};

		return Celebration {
			name,
			penance,
			color: Color::Green,
			rank,
			identifiers
		};
	}

	let _aug_weeks = NaiveDate::weeks_since(aug_sunday, sep_sunday);
	let sep_weeks = NaiveDate::weeks_since(sep_sunday, oct_sunday);
	let _oct_weeks = NaiveDate::weeks_since(oct_sunday, nov_sunday);
	let _nov_weeks = NaiveDate::weeks_since(nov_sunday, ly.next_advent);

	let (month_week_num, month) = if date.is_between(aug_sunday, sep_sunday) {
		(NaiveDate::weeks_since(aug_sunday, date) + 1, "August")
	} else if date.is_between(sep_sunday, oct_sunday) {
		let sep_week_num = NaiveDate::weeks_since(sep_sunday, date) + 1;
		if sep_weeks == 4 && sep_week_num == 4 {
			(sep_week_num + 1, "September")
		} else {
			(sep_week_num, "September")
		}
	} else if date.is_between(oct_sunday, nov_sunday) {
		(NaiveDate::weeks_since(oct_sunday, date) + 1, "October")
	} else {
		let nov_week_num = NaiveDate::weeks_since(nov_sunday, date) + 1;
		if (2..=4).contains(&nov_week_num) {
			(nov_week_num + 1, "November")
		} else {
			(nov_week_num, "November")
		}
	};

	let (name, color) = match date.weekday() {
		Sun => (format!("{} Sunday of {} and {} after Pentecost", month_week_num.ordinal(), month, week_num.ordinal()), Color::Green),
		Wed | Fri | Sat => {
			if month == "September" && month_week_num == 3 {
				(format!("Ember {} of September", date.weekday().fullname()), Color::Violet)
			} else {
				(format!("{} in the {} Week of {} and {} after Pentecost", date.weekday().fullname(), month_week_num.ordinal(), month, week_num.ordinal()), Color::Green)
			}
		},
		_ => (format!("{} in the {} Week of {} and {} after Pentecost", date.weekday().fullname(), month_week_num.ordinal(), month, week_num.ordinal()), Color::Green)
	};

	let name = match week_num {
		28 => match date.weekday() {
			Sun => format!("{} Sunday of {} and Last after Pentecost", month_week_num.ordinal(), month),
			_ => format!("{} in the {} Week of {} and Last after Pentecost", date.weekday().fullname(), month_week_num.ordinal(), month)
		},
		_ => name
	};

	let penance = match date.weekday() {
		Wed | Fri => {
			if month == "September" && month_week_num == 3 {
				Some(Penance::Fasting)
			} else {
				Some(Penance::Abstinence)
			}
		},
		Sat => {
			if month == "September" && month_week_num == 3 {
				Some(Penance::Vigil)
			} else {
				None
			}
		}
		_ => None
	};

	identifiers.push(Identifier {
		season: match month {
			"August" => Season::August,
			"September" => Season::September,
			"October" => Season::October,
			"November" => Season::November,
			_ => panic!("Unknown month {}", month)
		},
		week: month_week_num.to_string(),
		day: String::from(date.weekday().fullname())
	});

	Celebration {
		name,
		color,
		rank: if date.weekday() == Sun { Rank::Sunday } else { Rank::Feria },
		penance,
		identifiers
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_sunday_diff() {
		for x in 1600..=4000 {
			let ly = Kalendar::from_year(x).unwrap();
			let sundays_after_pentecost = NaiveDate::weeks_since(ly.pentecost.next_sunday().unwrap(), ly.next_advent);
			let sundays_after_epiphany = NaiveDate::weeks_since(ly.epiphany_sunday, ly.septuagesima);

			assert!((sundays_after_pentecost - 24) <= (6 - sundays_after_epiphany), "year {} had {} sundays after pentecost and {} after epiphany\ngiving {} sundays after epiphany to be resumed", x, sundays_after_pentecost, sundays_after_epiphany, sundays_after_pentecost - 24);
		}
	}
}