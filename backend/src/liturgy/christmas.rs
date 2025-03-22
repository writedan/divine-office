use crate::kalendar::Identifier;
use crate::liturgy::Liturgy;

use crate::kalendar::Season::*;

use std::collections::HashMap;
use std::path::PathBuf;

use chrono::{Weekday, Weekday::*};

pub fn resolve(iden: &Identifier) -> Liturgy {
    Liturgy {
        first_vespers: Some(first_vespers(iden)),
        first_compline: Some(compline(iden)),
        vigils: vigils(iden),
        matins: matins(iden),
        prime: prime(iden),
        terce: terce(iden),
        sext: sext(iden),
        none: none(iden),
        vespers: vespers(iden),
        compline: compline(iden),
    }
}

fn first_vespers(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
    HashMap::new()
}

fn compline(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
    let mut map = HashMap::new();

    map.insert(
        "order",
        ["compline", "order", "ordinary.lit"].iter().collect(),
    );
    map.insert("psalter", ["compline", "christmas.lit"].iter().collect());
    map.insert(
        "chapter",
        ["commons", "compline", "chapters", "ordinary.lit"]
            .iter()
            .collect(),
    );
    map.insert(
        "hymn",
        ["hymn", "te-lucis-ante-terminum", "ordinary.lit"]
            .iter()
            .collect(),
    );
    map.insert(
        "versicle",
        ["commons", "compline", "versicles", "ordinary.lit"]
            .iter()
            .collect(),
    );
    map.insert(
        "canticle",
        ["commons", "compline", "canticle", "christmas.lit"]
            .iter()
            .collect(),
    );
    map.insert(
        "collect",
        ["commons", "compline", "collect.lit"].iter().collect(),
    );
    map.insert(
        "anthem",
        [
            "commons",
            "compline",
            "anthems",
            "alma-redemptoris/christmas.lit",
        ]
        .iter()
        .collect(),
    );

    map.insert("kyrie", ["commons", "kyrie", "ii.gabc"].iter().collect());
    map.insert(
        "benedicamus",
        ["commons", "benedicamus", "ii.gabc"].iter().collect(),
    );

    return map;
}

fn vigils(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
    let mut map = HashMap::new();

    return map;
}

fn matins(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
    HashMap::new()
}

fn prime(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
    HashMap::new()
}

fn terce(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
    HashMap::new()
}

fn sext(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
    HashMap::new()
}

fn none(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
    HashMap::new()
}

fn vespers(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
    HashMap::new()
}
