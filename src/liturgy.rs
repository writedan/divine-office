use crate::kalendar::{Kalendar, Celebration, Rank::*, Identifier, Season::*};
use chrono::{NaiveDate, Weekday, Weekday::*};
use std::path::PathBuf;
use std::collections::HashMap;

mod ordinary;
mod commons;

impl Identifier {
	pub fn to_path(&self) -> PathBuf {
		["propers".to_string(), self.season.to_string(), self.week.to_lowercase(), self.day.to_lowercase()].iter().collect()
	}

	pub fn resolve(&self) -> Liturgy {
		let mut lit = match self.season {
			PostPentecost | PostEpiphany(_) | August | September | October | November => ordinary::resolve(self),
			_ => todo!("resolution of {}", self.season.to_string())
		};

		Self::verify_map(&mut lit.vigils);
		Self::verify_map(&mut lit.matins);
		Self::verify_map(&mut lit.prime);
		Self::verify_map(&mut lit.terce);
		Self::verify_map(&mut lit.sext);
		Self::verify_map(&mut lit.none);
		Self::verify_map(&mut lit.vespers);
		Self::verify_map(&mut lit.compline);

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

impl Liturgy {
	fn extend_helper(map1: &mut HashMap<&'static str,PathBuf>, map2: &HashMap<&'static str,PathBuf>) {
		for (key, value) in map2 {
			map1.insert(key.clone(), value.clone());
		}
	}

	pub fn extend(&mut self, other: &Liturgy) {
		Self::extend_helper(&mut self.vigils, &other.vigils);
		Self::extend_helper(&mut self.matins, &other.matins);
		Self::extend_helper(&mut self.prime, &other.prime);
		Self::extend_helper(&mut self.terce, &other.terce);
		Self::extend_helper(&mut self.sext, &other.sext);
		Self::extend_helper(&mut self.none, &other.none);
		Self::extend_helper(&mut self.vespers, &other.vespers);
		Self::extend_helper(&mut self.compline, &other.compline);
	}
}

pub fn resolve_hours(today: &Celebration, tomorrow: &Celebration) -> Liturgy  {
	let first_vespers = tomorrow.rank > today.rank && today.rank != StrongFeria;
	let idens: Vec<_> = today.identifiers(); // we reverse the vector since we will give precedence to what comes first
	let mut lit = idens[0].resolve();
	let idens: Vec<_> = idens.iter().map(|iden| iden.resolve()).collect();

	for x in idens.iter() {
		lit.extend(x);
	}

	lit.today_vespers = Some(!first_vespers);
	lit
}