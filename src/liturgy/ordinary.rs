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

	let commons = crate::liturgy::resolve_commons(iden).unwrap();
	let vigils = crate::liturgy::vigils_commons(iden).unwrap();

	let mut map: HashMap<&'static str, PathBuf> = HashMap::new();

	map.insert("order", [
		"vigils",
		"order",
		if day == Sun { "ordinary-sunday.lit" } else { "ordinary-feria.lit" }
	].iter().collect());

	map.insert("invitatory", [
		"invitatory",
		match day { 
			Sun => match iden.season {
				PostPentecost | PostEpiphany(_) => "preoccupemus.lit",
				August => "laudemus-jesum-christum.lit",
				September => "laudemus-nomen-domini.lit",
				October => "adaperiat-dominus.lit",
				November => "deus-rex-celestis.lit",
				_ => panic!("illegal season {:?}", iden.season)
			},
			Mon => "venite-exultemus.lit",
			Tue => "jubilemus-deo.lit",
			Wed => "in-manu-tua.lit",
			Thu => "adoremus-dominum.lit",
			Fri => "dominum-qui-fecit-nos.lit",
			Sat => "dominum-deum-nostrum.lit"
		}	
	].iter().collect());

	map.insert("hymn", [
		"hymn",
		match day {
			Sun => match iden.season {
				PostEpiphany(_) => "primo-dierum-omnium.lit",
				PostPentecost | August | September | October | November => "nocte-surgentes.lit",
				_ => panic!("illegal season {:?}", iden.season)
			},
			Mon => "somno-refectis-artubus.lit",
			Tue => "consors-paterni-luminis.lit",
			Wed => "rerum-creator-optime.lit",
			Thu => "nox-atra-rerum-contigit.lit",
			Fri => "tu-trinitatis-unitas.lit",
			Sat => "summe-deus-clementie.lit"
		}
	].iter().collect());

	map.insert("psalter", [
		"vigils",
		&(iden.day.to_lowercase() + ".lit")
	].iter().collect());

	map.extend(vigils);
	map.extend(commons);

	map
}

fn matins(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
	let day = iden.day.parse::<Weekday>().unwrap();
	let commons = crate::liturgy::resolve_commons(iden).unwrap();
	let mut map: HashMap<&'static str, PathBuf> = HashMap::new();

	map.insert("order", ["matins", "order", "ordinary.lit"].iter().collect());

	map.insert("psalter", ["matins", &(iden.day.to_lowercase() + ".lit")].iter().collect());

	map.insert("chapter", [
		"commons",
		"matins",
		"chapters",
		if day == Sun { "sunday.lit" } else { "feria.lit" }
	].iter().collect());

	map.insert("versicle", [
		"commons",
		"matins",
		"versicles",
		if day == Sun { "sunday.lit" } else { "feria.lit" }
	].iter().collect());

	map.insert("hymn", [
		"hymn",
		match day {
			Sun => if iden.season == PostEpiphany(true) || iden.season == PostEpiphany(false) { "eterne-rerum-conditor.lit" } else { "ecce-jam-noctis.lit" },
			Mon => "splendor-paterne-glorie.lit",
			Tue => "ales-diei-nuncius.lit",
			Wed => "nox-et-tenebre-nubila.lit",
			Thu => "lux-ecce-surgit-aurea.lit",
			Fri => "eterna-celi-gloria.lit",
			Sat => "aurora-jam-spargit-polum.lit"
		}
	].iter().collect());

	map.insert("canticle", if day == Sun {
		let mut path = iden.to_path();
		path.push("matins");
		path.push("benedictus.lit");
		path
	} else {
		[
			"commons",
			"matins",
			"benedictus",
			&iden.day.to_lowercase()
		].iter().collect()
	});

	map.extend(commons);
	map
}

fn prime(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
	let day = iden.day.parse::<Weekday>().unwrap();
	let commons = crate::liturgy::resolve_commons(iden).unwrap();
	let mut map: HashMap<&'static str, PathBuf> = HashMap::new();

	map.insert("hymn", [
		"hymn",
		"jam-lucis-orto-sidere",
		if day == Sun { "ordinary-sunday.lit" } else { "ordinary-feria.lit" }
	].iter().collect());

	map.insert("order", [
		"terce",
		"order",
		"ordinary.lit"
	].iter().collect());

	map.insert("psalter", [
		"prime",
		if day == Sun { "sunday.lit" } else { "feria.lit" }
	].iter().collect());

	map.insert("chapter", [
		"commons",
		"prime",
		"chapters",
		if day == Sun { "sunday.lit" } else { "feria.lit" }
	].iter().collect());

	map.insert("versicle", [
		"commons",
		"prime",
		"versicles",
		if day == Sun { "sunday.lit" } else { "feria.lit" }
	].iter().collect());

	map.insert("responsory", [
		"resp",
		"jesu-christe-fili-dei",
		if day == Sun { "ordinary-sunday.gabc" } else { "ordinary-feria.gabc" }
	].iter().collect());

	map.extend(commons);
	map
}

fn terce(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
	let day = iden.day.parse::<Weekday>().unwrap();
	let commons = crate::liturgy::resolve_commons(iden).unwrap();
	let mut map: HashMap<&'static str, PathBuf> = HashMap::new();

	map.insert("hymn", [
		"hymn",
		"nunc-sancte-nobis-spiritus",
		if day == Sun { "ordinary-sunday.lit" } else { "ordinary-feria.lit" }
	].iter().collect());

	map.insert("order", [
		"terce",
		"order",
		"ordinary.lit"
	].iter().collect());

	map.insert("psalter", [
		"terce",
		if day == Sun { "sunday.lit" } else { "feria.lit" }
	].iter().collect());

	map.insert("chapter", [
		"commons",
		"terce",
		"chapters",
		if day == Sun { "sunday.lit" } else { "feria.lit" }
	].iter().collect());

	map.insert("versicle", [
		"commons",
		"terce",
		"versicles",
		if day == Sun { "sunday.lit" } else { "feria.lit" }
	].iter().collect());

	map.insert("responsory", [
		"resp",
		if day == Sun { "inclina-cor-meum.gabc" } else { "sana-animam-meam.gabc" }
	].iter().collect());

	map.extend(commons);
	map
}

fn sext(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
	let day = iden.day.parse::<Weekday>().unwrap();
	let commons = crate::liturgy::resolve_commons(iden).unwrap();
	let mut map: HashMap<&'static str, PathBuf> = HashMap::new();

	map.insert("hymn", [
		"hymn",
		"rector-potens-verax",
		if day == Sun { "ordinary-sunday.lit" } else { "ordinary-feria.lit" }
	].iter().collect());

	map.insert("order", [
		"terce",
		"order",
		"ordinary.lit"
	].iter().collect());

	map.insert("psalter", [
		"sext",
		if day == Sun { "sunday.lit" } else { "feria.lit" }
	].iter().collect());

	map.insert("chapter", [
		"commons",
		"sext",
		"chapters",
		if day == Sun { "sunday.lit" } else { "feria.lit" }
	].iter().collect());

	map.insert("versicle", [
		"commons",
		"sext",
		"versicles",
		if day == Sun { "sunday.lit" } else { "feria.lit" }
	].iter().collect());

	map.insert("responsory", [
		"resp",
		if day == Sun { "in-eternum-domine.gabc" } else { "benedicam-dominum.gabc" }
	].iter().collect());

	map.extend(commons);
	map
}

fn none(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
	let day = iden.day.parse::<Weekday>().unwrap();
	let commons = crate::liturgy::resolve_commons(iden).unwrap();
	let mut map: HashMap<&'static str, PathBuf> = HashMap::new();

	map.insert("hymn", [
		"hymn",
		"rerum-deus-tenax",
		if day == Sun { "ordinary-sunday.lit" } else { "ordinary-feria.lit" }
	].iter().collect());

	map.insert("order", [
		"terce",
		"order",
		"ordinary.lit"
	].iter().collect());

	map.insert("psalter", [
		"none",
		if day == Sun { "sunday.lit" } else { "feria.lit" }
	].iter().collect());

	map.insert("chapter", [
		"commons",
		"none",
		"chapters",
		if day == Sun { "sunday.lit" } else { "feria.lit" }
	].iter().collect());

	map.insert("versicle", [
		"commons",
		"none",
		"versicles",
		if day == Sun { "sunday.lit" } else { "feria.lit" }
	].iter().collect());

	map.insert("responsory", [
		"resp",
		if day == Sun { "clamavi-in-toto-corde.gabc" } else { "redime-me-domine.gabc" }
	].iter().collect());

	map.extend(commons);
	map
}

fn vespers(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
	let day = iden.day.parse::<Weekday>().unwrap();
	let commons = crate::liturgy::resolve_commons(iden).unwrap();
	let mut map: HashMap<&'static str, PathBuf> = HashMap::new();

	map.insert("order", ["vespers", "order", "ordinary.lit"].iter().collect());

	map.insert("psalter", ["vespers", &(iden.day.to_lowercase() + ".lit")].iter().collect());

	map.insert("chapter", [
		"commons",
		"vespers",
		"chapters",
		if day == Sun { "sunday.lit" } else { "feria.lit" }
	].iter().collect());

	map.insert("versicle", [
		"commons",
		"vespers",
		"versicles",
		if day == Sun { "sunday.lit" } else { "feria.lit" }
	].iter().collect());

	map.insert("hymn", [
		"hymn",
		match day {
			Sun => "lucis-creator-optime.lit",
			Mon => "immense-celi-conditor.lit",
			Tue => "telluris-ingens-conditor.lit",
			Wed => "celi-deus-sanctissime.lit",
			Thu => "magne-deus-potentie.lit",
			Fri => "plasmator-hominis.lit",
			Sat => "o-lux-beata-trinitas.lit"
		}
	].iter().collect());

	map.insert("canticle", if day == Sun || day == Sat {
		let mut path = iden.to_path();
		path.push("vespers");
		path.push("magnificat.lit");
		path
	} else {
		[
			"commons",
			"vespers",
			"benedictus",
			&iden.day.to_lowercase()
		].iter().collect()
	});

	map.extend(commons);
	map
}

fn compline(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
	let day = iden.day.parse::<Weekday>().unwrap();
	let commons = crate::liturgy::resolve_commons(iden).unwrap();
	let mut map: HashMap<&'static str, PathBuf> = HashMap::new();

	map.insert("hymn", [
		"hymn",
		"te-lucis-ante-terminum.lit"
	].iter().collect());

	map.insert("order", [
		"compline",
		"order",
		"ordinary.lit"
	].iter().collect());

	map.insert("psalter", [
		"compline",
		"ordinary.lit"
	].iter().collect());

	map.insert("chapter", [
		"commons",
		"compline",
		"chapters",
		"ordinary.lit"
	].iter().collect());

	map.insert("versicle", [
		"commons",
		"compline",
		"versicles",
		"ordinary.lit"
	].iter().collect());

	map.insert("responsory", [
		"resp",
		"in-manus-tuas.gabc"
	].iter().collect());

	map.insert("anthem", [
		"commons",
		"compline",
		"anthems",
		"salve-regina.lit"
	].iter().collect());

	map.extend(commons);
	map
}