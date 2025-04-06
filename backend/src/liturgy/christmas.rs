use crate::kalendar::Identifier;
use crate::liturgy::Liturgy;


use std::collections::HashMap;
use std::path::PathBuf;


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
    HashMap::new()
}

fn vigils(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
    let mut map = HashMap::new();

    match iden.week.as_str() {
        "christmas-eve" => {
            map.insert("order", ["vigils", "order", match iden.day.as_str() {
                "Sunday" => "ordinary-sunday.lit",
                _ => "ordinary-feria.lit"
            }].iter().collect());

            map.insert("invitatory", ["invitatory", "hodie-scietis.lit"].iter().collect());

            map.insert("psalter", match iden.day.as_str() {
                "Sunday" => ["vigils", "advent-sunday.lit"].iter().collect(),
                _ => ["vigils", format!("{}.lit", iden.day.to_lowercase()).as_str()].iter().collect()
            });

            map.insert("hymn", ["hymn", "verbum-supernum-prodiens", "advent.lit"].iter().collect());

            if let "Sunday" = iden.day.as_str() {
                // use advent veriscles if christmas eve is 4th sunday of advent
                map.insert(
                    "versicle-1",
                    [
                        "commons",
                        "vigils",
                        "1st-nocturn",
                        "versicles",
                        "advent.lit",
                    ]
                    .iter()
                    .collect(),
                );

                map.insert(
                    "versicle-2",
                    [
                        "commons",
                        "vigils",
                        "2nd-nocturn",
                        "versicles",
                        "advent.lit",
                    ]
                    .iter()
                    .collect(),
                );

                map.insert(
                    "versicle-3",
                    [
                        "commons",
                        "vigils",
                        "3rd-nocturn",
                        "versicles",
                        "advent.lit",
                    ]
                    .iter()
                    .collect(),
                );
            } else {
                // special christmas versicle
                map.insert("versicle-1", ["commons", "vigils", "1st-nocturn", "versicles", "christmas.lit"].iter().collect());
            }
        },

        "christmas-day" => {},

        _ => panic!("Unknown christmas week {:?}", iden.week)
    };

    map
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
