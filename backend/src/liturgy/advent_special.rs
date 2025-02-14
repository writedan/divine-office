use crate::kalendar::Identifier;
use crate::liturgy::Liturgy;

use crate::kalendar::Season::*;

use std::collections::HashMap;
use std::path::PathBuf;

use chrono::{Weekday, Weekday::*};

pub fn resolve(iden: &Identifier) -> Liturgy {
    // AdventSpecial identifiers have day in the form of e.g. "1-Saturday", "1" here indicating December 17 (and so one, until the 23 being "7" and "Saturday" indicating the day of the week)
    // the former is used for vespers, the latter for matin
    let vespers_iden = Identifier {
        season: AdventSpecial,
        week: "o-antiphons".to_string(),
        day: iden.day.split('-').nth(0).unwrap().to_string(),
    };

    let matins_iden = Identifier {
        season: AdventSpecial,
        week: "o-antiphons".to_string(),
        day: iden.day.split('-').nth(1).unwrap().to_string(),
    };

    Liturgy {
        first_vespers: Some(vespers(
            &Identifier {
                // we have to use the previous day
                season: AdventSpecial,
                week: "o-antiphons".to_string(),
                day: (vespers_iden.day.parse::<usize>().unwrap() - 1).to_string(),
            },
            &iden.day,
            true,
        )),
        first_compline: Some(HashMap::new()),
        vigils: HashMap::new(),
        matins: matins(&matins_iden, &iden.day),
        prime: prime(&matins_iden, &iden.day),
        terce: terce(&matins_iden, &iden.day),
        sext: sext(&matins_iden, &iden.day),
        none: none(&matins_iden, &iden.day),
        vespers: vespers(&vespers_iden, &iden.day, false),
        compline: HashMap::new(),
    }
}

fn vespers(
    iden: &Identifier,
    fullid: &String,
    first_vespers: bool,
) -> HashMap<&'static str, PathBuf> {
    let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
    map.insert(
        "canticle",
        iden.to_path().join("vespers").join("magnificat.lit"),
    );
    if fullid != "1-Sunday" {
        let day = fullid.split('-').nth(1).unwrap().to_lowercase();
        // see note below for special_matins
        if fullid.ends_with("Sunday") && !first_vespers {
            map.insert(
                "psalter",
                [
                    "propers",
                    "advent",
                    "o-antiphons",
                    &day,
                    "vespers",
                    "psalter.lit",
                ]
                .iter()
                .collect(),
            );
        }
    }
    return map;
}

fn matins(iden: &Identifier, fullid: &String) -> HashMap<&'static str, PathBuf> {
    let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
    if fullid != "1-Sunday" {
        // we do not override the psalter for the 3rd Sunday of advent, which is the only case this can occur
        map.insert("psalter", iden.to_path().join("matins").join("psalter.lit"));
    }
    return map;
}

fn prime(iden: &Identifier, fullid: &String) -> HashMap<&'static str, PathBuf> {
    let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
    if fullid != "1-Sunday" {
        // we do not override the psalter for the 3rd Sunday of advent, which is the only case this can occur
        map.insert("psalter", iden.to_path().join("prime").join("psalter.lit"));
    }
    return map;
}

fn terce(iden: &Identifier, fullid: &String) -> HashMap<&'static str, PathBuf> {
    let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
    if fullid != "1-Sunday" {
        // we do not override the psalter for the 3rd Sunday of advent, which is the only case this can occur
        map.insert("psalter", iden.to_path().join("terce").join("psalter.lit"));
    }
    return map;
}

fn sext(iden: &Identifier, fullid: &String) -> HashMap<&'static str, PathBuf> {
    let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
    if fullid != "1-Sunday" {
        // we do not override the psalter for the 3rd Sunday of advent, which is the only case this can occur
        map.insert("psalter", iden.to_path().join("sext").join("psalter.lit"));
    }
    return map;
}

fn none(iden: &Identifier, fullid: &String) -> HashMap<&'static str, PathBuf> {
    let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
    if fullid != "1-Sunday" {
        // we do not override the psalter for the 3rd Sunday of advent, which is the only case this can occur
        map.insert("psalter", iden.to_path().join("none").join("psalter.lit"));
    }
    return map;
}
