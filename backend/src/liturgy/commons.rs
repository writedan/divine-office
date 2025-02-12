use crate::kalendar::{Identifier, Season::*};
use chrono::{Weekday, Weekday::*};
use std::path::PathBuf;
use std::collections::HashMap;

pub fn resolve(iden: &Identifier) -> Option<HashMap<&'static str, PathBuf>> {
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
			Advent => if day == Sun { "xvii.gabc" } else { "xviii.gabc" },
			_ => todo!("kyrie for {:?}", iden.season)
		}
	].iter().collect());

	map.insert("benedicamus", [
		"commons",
		"benedicamus",
		match iden.season {
			PostPentecost | PostEpiphany(_) | August | September | October | November => if day == Sun { "xi.gabc" } else { "xvi.gabc" },
			Advent => if day == Sun { "xvii.gabc" } else { "xviii.gabc" },
			_ => todo!("benedicamus for {:?}", iden.season)
		}
	].iter().collect());

	Some(map)
}

pub fn vigils(iden: &Identifier) -> Option<HashMap<&'static str, PathBuf>> {
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

	map.insert("gospel", path.join("gospel.lit"));

	Some(map)
}