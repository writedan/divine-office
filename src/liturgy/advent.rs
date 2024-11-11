use crate::kalendar::Identifier;
use crate::liturgy::Liturgy;

use crate::kalendar::Season::*;

use std::collections::HashMap;
use std::path::PathBuf;

use chrono::{Weekday, Weekday::*};

pub fn resolve(iden: &Identifier) -> Liturgy {
	Liturgy {
		today_vespers: None,
		vigils: vigils(iden),
		matins: matins(iden),
		prime: prime(iden),
		terce: terce(iden),
		sext: sext(iden),
		none: none(iden),
		vespers: vespers(iden),
		compline: compline(iden)
	}
}

fn vigils(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
	let day = iden.day.parse::<Weekday>().unwrap();

	let commons = crate::liturgy::commons::resolve(iden).unwrap();
	let vigils = crate::liturgy::commons::vigils(iden).unwrap();

	let mut map: HashMap<&'static str, PathBuf> = HashMap::new();

	map.insert("order", [
		"vigils",
		"order",
		if day == Sun { "ordinary-sunday.lit" } else { "ordinary-feria.lit" }
	].iter().collect());

	map.insert("invitatory", match day {
		Sun => iden.to_path().join("vigils").join("invitatory.lit"),
		_ => ["invitatory", "regem-venturum.lit"].iter().collect()
	});

	map.insert("hymn", ["hymn", "verbum-supernum-prodiens", "advent.lit"].iter().collect());

	map.insert("psalter", match day {
		Sun => ["vigils", "advent-sunday.lit"].iter().collect(),
		_ => ["vigils", &(iden.day.to_lowercase() + ".lit")].iter().collect()
	});

	map.extend(vigils);
	map.extend(commons);

	return map;
}

fn matins(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
	let day = iden.day.parse::<Weekday>().unwrap();

	let commons = crate::liturgy::commons::resolve(iden).unwrap();

	let mut map: HashMap<&'static str, PathBuf> = HashMap::new();

	map.insert("order", ["matins", "order", "ordinary.lit"].iter().collect());

	map.insert("psalter", match day {
		Sun => iden.to_path().join("psalter.lit"),
		_ => ["matins", &(iden.day.to_lowercase() + ".lit")].iter().collect()
	});

	

	map.extend(commons);

	return map;
}

fn prime(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
	let day = iden.day.parse::<Weekday>().unwrap();

	let commons = crate::liturgy::commons::resolve(iden).unwrap();

	let mut map: HashMap<&'static str, PathBuf> = HashMap::new();

	map.extend(commons);

	return map;
}

fn terce(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
	let day = iden.day.parse::<Weekday>().unwrap();

	let commons = crate::liturgy::commons::resolve(iden).unwrap();

	let mut map: HashMap<&'static str, PathBuf> = HashMap::new();

	map.extend(commons);

	return map;
}

fn sext(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
	let day = iden.day.parse::<Weekday>().unwrap();

	let commons = crate::liturgy::commons::resolve(iden).unwrap();

	let mut map: HashMap<&'static str, PathBuf> = HashMap::new();

	map.extend(commons);

	return map;
}

fn none(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
	let day = iden.day.parse::<Weekday>().unwrap();

	let commons = crate::liturgy::commons::resolve(iden).unwrap();

	let mut map: HashMap<&'static str, PathBuf> = HashMap::new();

	map.extend(commons);

	return map;
}

fn vespers(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
	let day = iden.day.parse::<Weekday>().unwrap();

	let commons = crate::liturgy::commons::resolve(iden).unwrap();

	let mut map: HashMap<&'static str, PathBuf> = HashMap::new();

	map.extend(commons);

	return map;
}

fn compline(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
	let day = iden.day.parse::<Weekday>().unwrap();

	let commons = crate::liturgy::commons::resolve(iden).unwrap();

	let mut map: HashMap<&'static str, PathBuf> = HashMap::new();

	map.extend(commons);

	return map;
}
