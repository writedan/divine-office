use crate::kalendar::{Celebration, Identifier, Rank::*, Season::*};
use chrono::{Weekday, Weekday::*};
use std::collections::HashMap;
use std::path::PathBuf;

mod advent;
/// O antiphons
mod advent_special;
mod christmas;
mod commons;
mod ordinary;

impl Identifier {
    pub fn to_path(&self) -> PathBuf {
        [
            "propers".to_string(),
            self.season.to_string(),
            self.week.to_lowercase(),
            self.day.to_lowercase(),
        ]
        .iter()
        .collect()
    }

    pub fn resolve(&self) -> Liturgy {
        let mut lit = match self.season {
            PostPentecost | PostEpiphany(_) | August | September | October | November => {
                ordinary::resolve(self)
            }
            Advent => advent::resolve(self),
            AdventSpecial => advent_special::resolve(self),
            Christmas => christmas::resolve(self),
            _ => todo!("resolution of {} for {:?}", self.season.to_string(), self),
        };

        if let August | September | October | November = self.season {
            lit.vigils.remove("collect");
            lit.vigils.remove("gospel");
            lit.vigils.remove("lesson-7");
            lit.vigils.remove("lesson-8");
            lit.vigils.remove("lesson-9");
            lit.matins.remove("collect");
            lit.prime.remove("collect");
            lit.terce.remove("collect");
            lit.sext.remove("collect");
            lit.none.remove("collect");
            lit.vespers.remove("collect");
            lit.compline.remove("collect");
            let day = self.day.parse::<Weekday>().ok().unwrap();
            if day == Sun {
                lit.vespers.remove("canticle");
            }
        }

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
    pub first_vespers: Option<HashMap<&'static str, PathBuf>>,
    pub first_compline: Option<HashMap<&'static str, PathBuf>>,
    pub vigils: HashMap<&'static str, PathBuf>,
    pub matins: HashMap<&'static str, PathBuf>,
    pub prime: HashMap<&'static str, PathBuf>,
    pub terce: HashMap<&'static str, PathBuf>,
    pub sext: HashMap<&'static str, PathBuf>,
    pub none: HashMap<&'static str, PathBuf>,
    pub vespers: HashMap<&'static str, PathBuf>,
    pub compline: HashMap<&'static str, PathBuf>,
}

impl Liturgy {
    fn extend_helper(
        map1: &mut HashMap<&'static str, PathBuf>,
        map2: &HashMap<&'static str, PathBuf>,
    ) {
        for (key, value) in map2 {
            if !map1.contains_key(key) {
                map1.insert(key.clone(), value.clone());
            }
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

        if self.first_vespers.is_some() && other.first_vespers.is_some() {
            Self::extend_helper(
                self.first_vespers.as_mut().unwrap(),
                other.first_vespers.as_ref().unwrap(),
            );
        }

        if self.first_compline.is_some() && other.first_compline.is_some() {
            Self::extend_helper(
                self.first_compline.as_mut().unwrap(),
                other.first_compline.as_ref().unwrap(),
            );
        }
    }
}

pub fn first_vespers(today: &Celebration, tomorrow: &Celebration) -> bool {
    (tomorrow.rank > today.rank && today.rank != StrongFeria && tomorrow.rank != StrongFeria) || today.rank == Eve
}

pub fn resolve_hours(today: &Celebration, tomorrow: &Celebration) -> Liturgy {
    let first_vespers = first_vespers(today, tomorrow);
    let idens: Vec<_> = today.identifiers();
    let tomorrow_idens: Vec<_> = tomorrow.identifiers();

    let mut lit = idens[0].resolve();
    let mut tomorrow_lit = tomorrow_idens[0].resolve();

    let idens: Vec<_> = idens.iter().map(|iden| iden.resolve()).collect();
    let tomorrow_idens: Vec<_> = tomorrow_idens.iter().map(|iden| iden.resolve()).collect();

    for x in idens.iter() {
        lit.extend(x);
    }

    for x in tomorrow_idens.iter() {
        tomorrow_lit.extend(x);
    }

    if (first_vespers) {
        lit.vespers = tomorrow_lit
            .first_vespers
            .expect(format!("expected to find first_vespers for tomorrow={:?}", tomorrow).as_str());

        lit.compline = tomorrow_lit.first_compline.expect(
            format!(
                "expected to find first_compline for tomorrow={:?}",
                tomorrow
            )
            .as_str(),
        );
    }

    lit
}
