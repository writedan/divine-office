use crate::kalendar::Identifier;
use crate::liturgy::Liturgy;

use crate::kalendar::Season::*;

use std::collections::HashMap;
use std::path::PathBuf;

pub fn resolve(iden: &Identifier) -> Liturgy {
    match iden.day.as_str() {
        "Thursday" | "Friday" | "Saturday" => Liturgy {
            first_vespers: None,
            first_compline: None,
            vigils: triduum::vigils(iden),
            matins: triduum::matins(iden),
            prime: triduum::prime(iden),
            terce: triduum::terce(iden),
            sext: triduum::sext(iden),
            none: triduum::none(iden),
            vespers: triduum::vespers(iden),
            compline: triduum::compline(iden)
        },

        _ => Liturgy {
            first_vespers: if iden.day.as_str() == "Sunday" {
                Some(passion::first_vespers(iden))
            } else {
                None
            },
            first_compline: if iden.day.as_str() == "Sunday" {
                Some(passion::compline(iden))
            } else {
                None
            },
            vigils: passion::vigils(iden),
            matins: passion::matins(iden),
            prime: passion::prime(iden),
            terce: passion::terce(iden),
            sext: passion::sext(iden),
            none: passion::none(iden),
            vespers: passion::vespers(iden),
            compline: passion::compline(iden)
        }
    }
}

mod passion {
    use super::*;

    pub fn first_vespers(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
        let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
        map.insert("order", ["matins", "order", "penitential.lit"].iter().collect());
        map.insert("collect", iden.to_path().join("vespers").join("collect.lit"));
        return map;
    }

    pub fn vigils(iden: &Identifier) -> HashMap<&'static str,PathBuf> {
        let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
        return map;
    }

    pub fn matins(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
        let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
        return map;
    }

    pub fn prime(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
        let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
        return map;
    }

    pub fn terce(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
        let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
        return map;
    }

    pub fn sext(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
        let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
        return map;
    }

    pub fn none(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
        let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
        return map;
    }

    pub fn vespers(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
        let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
        return map;
    }

    pub fn compline(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
        let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
        return map;
    }
}

mod triduum {
    use super::*;

    pub fn vigils(iden: &Identifier) -> HashMap<&'static str,PathBuf> {
        let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
        return map;
    }

    pub fn matins(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
        let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
        return map;
    }

    pub fn prime(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
        let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
        return map;
    }

    pub fn terce(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
        let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
        return map;
    }

    pub fn sext(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
        let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
        return map;
    }

    pub fn none(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
        let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
        return map;
    }

    pub fn vespers(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
        let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
        return map;
    }

    pub fn compline(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
        let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
        return map;
    }
}