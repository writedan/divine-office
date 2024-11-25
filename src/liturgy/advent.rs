use crate::kalendar::Identifier;
use crate::liturgy::Liturgy;

use crate::kalendar::Season::*;

use std::collections::HashMap;
use std::path::PathBuf;

use chrono::{Weekday, Weekday::*};

pub fn resolve(iden: &Identifier) -> Liturgy {
	Liturgy {
		first_vespers: if "Sunday" == iden.day { Some(first_vespers(iden)) } else { None} ,
		first_compline: if "Sunday" == iden.day { Some(compline(iden)) } else { None },
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

fn first_vespers(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
	let day = iden.day.parse::<Weekday>().unwrap();

	let commons = crate::liturgy::commons::resolve(iden).unwrap();

	let mut map: HashMap<&'static str, PathBuf> = HashMap::new();

	map.insert("order", ["matins", "order", "ordinary.lit"].iter().collect());
	map.insert("psalter", ["vespers", "saturday.lit"].iter().collect());
	map.insert("chapter", iden.to_path().join("vespers").join("chapter.lit"));
	map.insert("hymn", ["hymn", "conditor-alme-syderum", "advent.lit"].iter().collect());
	map.insert("versicle", ["commons", "vespers", "versicles", "advent.lit"].iter().collect());
	map.insert("canticle", iden.to_path().join("vespers").join("magnificat.lit"));

	map.extend(crate::liturgy::commons::resolve(iden).unwrap());
	map
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
		Sun => iden.to_path().join("matins").join("psalter.lit"),
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

	map.insert("order", ["compline", "order", "ordinary.lit"].iter().collect());
	map.insert("psalter", ["compline", "ordinary.lit"].iter().collect());
	map.insert("chapter", ["commons", "compline", "chapters", "ordinary.lit"].iter().collect());
	map.insert("hymn", ["hymn", "te-lucis-ante-terminum", "ordinary.lit"].iter().collect());
	map.insert("versicle", ["commons", "compline", "versicles", "ordinary.lit"].iter().collect());
	map.insert("canticle", ["commons", "compline", "canticle", "advent.lit"].iter().collect());
	map.insert("anthem", ["commons", "compline", "anthems", "alma-redemptoris/advent.lit"].iter().collect());

	map.extend(commons);

	map.insert("collect", ["commons", "compline", "collect.lit"].iter().collect());

	return map;
}
