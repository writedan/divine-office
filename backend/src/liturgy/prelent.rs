use crate::kalendar::{Identifier, Season::*};
use crate::liturgy::Liturgy;
use std::collections::HashMap;
use std::path::PathBuf;
use chrono::{Weekday, Weekday::*};

pub fn resolve(iden: &Identifier) -> Liturgy {
    Liturgy {
        first_vespers: if "Sunday" == iden.day {
            Some(first_vespers(iden))
        } else {
            None
        },
        first_compline: if "Sunday" == iden.day {
            Some(compline(iden))
        } else {
            None
        },
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

fn commons(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
    let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
    map.extend(crate::liturgy::commons::resolve(iden).unwrap());
    return map;
}

fn first_vespers(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
    let mut map = HashMap::new();
    let day = iden.day.parse::<Weekday>().unwrap();

    map.insert("order", ["matins", "order", "penitential.lit"].iter().collect());

    map.insert("psalter", ["vespers", "saturday.lit"].iter().collect());

    map.insert("chapter", iden.to_path().join("vespers").join("chapter.lit"));

    map.insert(
        "hymn",
        [
            "hymn",
            "o-lux-beata-trinitas",
            match iden.season {
                PreLent(true) => "bvm.lit",
                PreLent(false) => "ordinary.lit",
                _ => panic!("Illegal season: {:?}", iden.season)
            },
        ]
        .iter()
        .collect(),
    );

    map.insert(
        "versicle",
        [
            "commons",
            "vespers",
            "versicles",
            if day == Sun {
                "sunday.lit"
            } else {
                "feria.lit"
            },
        ]
        .iter()
        .collect(),
    );

    map.insert(
        "canticle",
        iden.to_path().join("1st-vespers").join("magnificat.lit"),
    );

    map.insert("penitential-psalm", ["commons", "vespers", "penitential-psalm.lit"].iter().collect());
    
    map.extend(commons(iden));
    return map;
}

fn compline(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
    let mut map = HashMap::new();

    map.insert("order", ["compline", "order", "penitential.lit"].iter().collect());

    map.insert("psalter", ["compline", "ordinary.lit"].iter().collect());

    map.insert(
        "chapter",
        ["commons", "compline", "chapters", "ordinary.lit"]
            .iter()
            .collect(),
    );

    map.insert(
        "hymn",
        ["hymn", "te-lucis-ante-terminum", match iden.season {
            PreLent(true) => "bvm.lit",
            PreLent(false) => "ordinary.lit",
            _ => panic!("Illegal season: {:?}", iden.season)
        }]
        .iter()
        .collect()
    );

    map.insert(
        "versicle",
        ["commons", "compline", "versicles", "ordinary.lit"]
            .iter()
            .collect(),
    );

    map.insert(
        "canticle",
        ["commons", "compline", "canticle", "ordinary.lit"]
            .iter()
            .collect(),
    );

    map.insert("anthem", [
        "commons",
        "compline",
        "anthems",
        match iden.season {
            PreLent(true) => "alma-redemptoris/christmas.lit",
            PreLent(false) => "ave-regina-caelorum.lit",
            _ => panic!("Illegal season: {:?}", iden.season)
        }
    ].iter().collect());

    map.extend(commons(iden));

    map.insert(
        "collect",
        ["commons", "compline", "collect.lit"].iter().collect(),
    );

    return map;
}

fn vigils(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
    let mut map = HashMap::new();
    let vigils = crate::liturgy::commons::vigils(iden).unwrap();
    let day = iden.day.parse::<Weekday>().unwrap();

    map.extend(vigils);
    map.extend(commons(iden));
    map.extend(crate::liturgy::ordinary::resolve(iden).vigils);

    map.insert("order", ["vigils", "order", match day {
            Sun => "penitential-sunday.lit",
            _ => "penitential-feria.lit"
        }
    ].iter().collect());

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
