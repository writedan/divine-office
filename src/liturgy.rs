use crate::kalendar::{Kalendar, Celebration, Rank::*, Identifier, Season::*};
use chrono::{NaiveDate, Weekday, Weekday::*};
use std::path::PathBuf;
use std::collections::HashMap;

mod ordinary;

impl Identifier {
	pub fn to_path(&self) -> PathBuf {
		["propers".to_string(), self.season.to_string(), self.week.to_lowercase(), self.day.to_lowercase()].iter().collect()
	}

	pub fn resolve(&self) -> Liturgy {
		let mut lit = match self.season {
			PostPentecost | PostEpiphany(_) | August | September | October | November => ordinary::resolve(self),
			_ => todo!("resolution of {}", self.season.to_string())
		};

		// Self::verify_map(&mut lit.vigils);
		// Self::verify_map(&mut lit.matins);
		// Self::verify_map(&mut lit.prime);
		// Self::verify_map(&mut lit.terce);
		// Self::verify_map(&mut lit.sext);
		// Self::verify_map(&mut lit.none);
		// Self::verify_map(&mut lit.vespers);
		// Self::verify_map(&mut lit.compline);

		lit
	}

	fn verify_map(map: &mut HashMap<&'static str, PathBuf>) {
		let mut to_remove = Vec::new();
		for (key, value) in &*map {
			if !value.exists() {
				to_remove.push(key.to_owned());
			}
		}

		for key in to_remove.iter() {
			map.remove(key);
		}
	}
}

#[derive(Debug)]
pub struct Liturgy {
	pub today_vespers: Option<bool>, // whether vespers+compline are of today (true) or tomorrow (false). If false, we must display it until tomorrow's liturgy
	pub vigils: HashMap<&'static str, PathBuf>,
	pub matins: HashMap<&'static str, PathBuf>,
	pub prime: HashMap<&'static str, PathBuf>,
	pub terce: HashMap<&'static str, PathBuf>,
	pub sext: HashMap<&'static str, PathBuf>,
	pub none: HashMap<&'static str, PathBuf>,
	pub vespers: HashMap<&'static str, PathBuf>,
	pub compline: HashMap<&'static str, PathBuf>
}

pub fn resolve_hours(today: &Celebration, tomorrow: &Celebration) -> Liturgy  {
	let first_vespers = tomorrow.rank > today.rank && today.rank != StrongFeria;
	let idens: Vec<_> = today.identifiers(); // we reverse the vector since we will give precedence to what comes first
	idens[0].resolve()
}

pub fn resolve_commons(iden: &Identifier) -> Option<HashMap<&'static str, PathBuf>> {
	let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
	let day = iden.day.parse::<Weekday>().ok()?;

	let day_str = &iden.day.to_lowercase();

	map.insert("collect", [
		"propers",
		&iden.season.to_string().to_lowercase(),
		&iden.week,
		if iden.season == Lent { day_str } else { "" },
		"collect.lit"
	].iter().collect());

	map.insert("kyrie", [
		"commons",
		"kyrie",
		match iden.season {
			PostPentecost | PostEpiphany(_) | August | September | October | November => if day == Sun { "xi.gabc" } else { "xvi.gabc" },
			_ => todo!("kyrie for {:?}", iden.season)
		}
	].iter().collect());

	map.insert("doxology", [
		"hymn",
		"doxology",
		match iden.season {
			PostPentecost | August | September | October | November | PostEpiphany(false) => "post-purification.lit",
			_ => todo!("doxology for {:?}", iden.season)
		}
	].iter().collect());

	Some(map)
}

pub fn vigils_commons(iden: &Identifier) -> Option<HashMap<&'static str, PathBuf>> {
	let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
	let day = iden.day.parse::<Weekday>().ok()?;
	match day {
		Sun => {
			map.insert("absolution-1", "commons/vigils/1st-nocturn/absolution.gabc".into());
			map.insert("blessing-1", "commons/vigils/1st-nocturn/blessing-1.gabc".into());
			map.insert("blessing-2", "commons/vigils/1st-nocturn/blessing-2.gabc".into());
			map.insert("blessing-3", "commons/vigils/1st-nocturn/blessing-3.gabc".into());

			map.insert("absolution-2", "commons/vigils/2nd-nocturn/absolution.gabc".into());
			map.insert("blessing-4", "commons/vigils/2nd-nocturn/blessing-1.gabc".into());
			map.insert("blessing-5", "commons/vigils/2nd-nocturn/blessing-2.gabc".into());
			map.insert("blessing-6", "commons/vigils/2nd-nocturn/blessing-3.gabc".into());

			map.insert("absolution-3", "commons/vigils/3rd-nocturn/absolution.gabc".into());
			map.insert("blessing-7", "commons/vigils/3rd-nocturn/blessing-1.gabc".into());
			map.insert("blessing-8", "commons/vigils/3rd-nocturn/blessing-2.gabc".into());
			map.insert("blessing-9", "commons/vigils/3rd-nocturn/blessing-3.gabc".into());
		},

		Mon | Thu => {
			map.insert("absolution-1", "commons/vigils/1st-nocturn/absolution.gabc".into());
			map.insert("blessing-1", "commons/vigils/1st-nocturn/blessing-1.gabc".into());
			map.insert("blessing-2", "commons/vigils/1st-nocturn/blessing-2.gabc".into());
			map.insert("blessing-3", "commons/vigils/1st-nocturn/blessing-3.gabc".into());
		},

		Tue | Fri => {
			map.insert("absolution-1", "commons/vigils/2nd-nocturn/absolution.gabc".into());
			map.insert("blessing-1", "commons/vigils/2nd-nocturn/blessing-1.gabc".into());
			map.insert("blessing-2", "commons/vigils/2nd-nocturn/blessing-2.gabc".into());
			map.insert("blessing-3", "commons/vigils/2nd-nocturn/blessing-3.gabc".into());
		},

		Wed | Sat => {
			map.insert("absolution-1", "commons/vigils/3rd-nocturn/absolution.gabc".into());
			map.insert("blessing-1", "commons/vigils/3rd-nocturn/blessing-1.gabc".into());
			map.insert("blessing-2", "commons/vigils/3rd-nocturn/blessing-2.gabc".into());
			map.insert("blessing-3", "commons/vigils/3rd-nocturn/blessing-3.gabc".into());
		}
	};

	let LESSON_CODES: [&str; 9] = [
	    "lesson-1",
	    "lesson-2",
	    "lesson-3",
	    "lesson-4",
	    "lesson-5",
	    "lesson-6",
	    "lesson-7",
	    "lesson-8",
	    "lesson-9",
	]; // necessary to avoid static reference dropping below

	let path = iden.to_path();
	for (i, &code) in LESSON_CODES.iter().enumerate() {
		let mut lesson_path = path.clone();
		lesson_path.push("vigils");
		lesson_path.push("lessons");
		lesson_path.push((i + 1).to_string());
		lesson_path.set_extension("lit");
		map.insert(code, lesson_path);
	}

	Some(map)
}