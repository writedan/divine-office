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
	map.insert("chapter", iden.to_path().join("1st-vespers").join("chapter.lit"));
	map.insert("hymn", ["hymn", "conditor-alme-syderum", "advent.lit"].iter().collect());
	map.insert("versicle", ["commons", "vespers", "versicles", "advent.lit"].iter().collect());
	map.insert("canticle", iden.to_path().join("1st-vespers").join("magnificat.lit"));

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

	map.insert("versicle-1", ["commons", "vigils", match day {
		Sun | Mon | Thu => "1st-nocturn",
		Tue | Fri => "2nd-nocturn",
		Wed | Sat => "3rd-nocturn"
	}, "versicles", "advent.lit"].iter().collect());

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

	map.insert("chapter", match day {
		Sun => iden.to_path().join("matins").join("chapter.lit"),
		_ => ["commons", "matins", "chapters", "advent-feria.lit"].iter().collect()
	});

	map.insert("hymn", ["hymn", "vox-clara-ecce-intonat", "advent.lit"].iter().collect());

	map.insert("versicle", ["commons", "matins", "versicles", "advent.lit"].iter().collect());

	map.insert("canticle", iden.to_path().join("matins").join("benedictus.lit"));

	map.extend(commons);

	return map;
}

fn prime(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
	let day = iden.day.parse::<Weekday>().unwrap();

	let commons = crate::liturgy::commons::resolve(iden).unwrap();

	let mut map: HashMap<&'static str, PathBuf> = HashMap::new();

	map.insert("order", ["terce", "order", "ordinary.lit"].iter().collect());

	map.insert("hymn", ["hymn", "jam-lucis-orto-sidere", "advent-sunday.lit"].iter().collect());

	map.insert("psalter", iden.to_path().join("prime").join("psalter.lit"));

	map.insert("chapter", match day {
		Sun => ["commons", "prime", "chapters", "sunday.lit"],
		_ => ["commons", "prime", "chapters", "feria.lit"]
	}.iter().collect());

	map.insert("responsory", match day {
		Sun => ["resp", "jesu-christe-fili-dei", "advent-sunday.gabc"],
		_ => ["resp", "jesu-christe-fili-dei", "advent-feria.gabc"]
	}.iter().collect());

	map.insert("versicle", ["commons", "prime", "versicles", "ordinary.lit"].iter().collect());

	map.extend(commons);

	map.insert("collect", ["commons", "prime", "collect.lit"].iter().collect());

	return map;
}

fn terce(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
	let day = iden.day.parse::<Weekday>().unwrap();

	let commons = crate::liturgy::commons::resolve(iden).unwrap();

	let mut map: HashMap<&'static str, PathBuf> = HashMap::new();

	map.insert("order", ["terce", "order", "ordinary.lit"].iter().collect());

	map.insert("hymn", ["hymn", "nunc-sancte-nobis-spiritus", "ordinary-sunday.lit"].iter().collect());

	map.insert("psalter", iden.to_path().join("terce").join("psalter.lit"));

	map.insert("chapter", iden.to_path().join("terce").join("chapter.lit"));

	map.insert("versicle", ["commons", "terce", "versicles", "advent.lit"].iter().collect());

	map.extend(commons);

	return map;
}

fn sext(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
	let day = iden.day.parse::<Weekday>().unwrap();

	let commons = crate::liturgy::commons::resolve(iden).unwrap();

	let mut map: HashMap<&'static str, PathBuf> = HashMap::new();

	map.insert("order", ["terce", "order", "ordinary.lit"].iter().collect());

	map.insert("hymn", ["hymn", "rector-potens-verax", "ordinary-sunday.lit"].iter().collect());

	map.insert("psalter", iden.to_path().join("sext").join("psalter.lit"));

	map.insert("chapter", iden.to_path().join("sext").join("chapter.lit"));

	map.insert("versicle", ["commons", "sext", "versicles", "advent.lit"].iter().collect());

	map.extend(commons);

	return map;
}

fn none(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
	let day = iden.day.parse::<Weekday>().unwrap();

	let commons = crate::liturgy::commons::resolve(iden).unwrap();

	let mut map: HashMap<&'static str, PathBuf> = HashMap::new();

	map.insert("order", ["terce", "order", "ordinary.lit"].iter().collect());

	map.insert("hymn", ["hymn", "rerum-deus-tenax", "ordinary-sunday.lit"].iter().collect());

	map.insert("psalter", iden.to_path().join("none").join("psalter.lit"));

	map.insert("chapter", iden.to_path().join("none").join("chapter.lit"));

	map.insert("versicle", ["commons", "none", "versicles", "advent.lit"].iter().collect());

	map.extend(commons);

	return map;
}

fn vespers(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
	let day = iden.day.parse::<Weekday>().unwrap();

	let commons = crate::liturgy::commons::resolve(iden).unwrap();

	let mut map: HashMap<&'static str, PathBuf> = HashMap::new();

	map.insert("order", ["matins", "order", "ordinary.lit"].iter().collect());

	map.insert("psalter", match day {
		Sun => iden.to_path().join("vespers").join("psalter.lit"),
		_ => ["vespers", &(iden.day.to_lowercase() + ".lit")].iter().collect()
	});

	map.insert("chapter", iden.to_path().join("vespers").join("chapter.lit"));

	map.insert("hymn", ["hymn", "conditor-alme-syderum", "advent.lit"].iter().collect());

	map.insert("versicle", ["commons", "vespers", "versicles", "advent.lit"].iter().collect());

	map.insert("canticle", iden.to_path().join("vespers").join("magnificat.lit"));

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
