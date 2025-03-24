use crate::kalendar::{Identifier, Season::*};
use chrono::{Weekday, Weekday::*};
use std::collections::HashMap;
use std::path::PathBuf;

pub fn resolve(iden: &Identifier) -> Option<HashMap<&'static str, PathBuf>> {
    let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
    let day = iden.day.parse::<Weekday>().ok()?;

    let day_str = &iden.day.to_lowercase();

    map.insert(
        "collect",
        [
            "propers",
            &iden.season.to_string().to_lowercase(),
            &iden.week,
            if iden.season == Lent { day_str } else { "" },
            "collect.lit",
        ]
        .iter()
        .collect(),
    );

    map.insert(
        "kyrie",
        [
            "commons",
            "kyrie",
            match iden.season {
                PreLent(_) | PostPentecost | PostEpiphany(_) | August | September | October | November => {
                    if day == Sun {
                        "xi.gabc"
                    } else {
                        "xvi.gabc"
                    }
                }
                Advent => {
                    if day == Sun {
                        "xvii.gabc"
                    } else {
                        "xviii.gabc"
                    }
                }
                _ => todo!("kyrie for {:?}", iden.season),
            },
        ]
        .iter()
        .collect(),
    );

    map.insert(
        "benedicamus",
        [
            "commons",
            "benedicamus",
            match iden.season {
                PreLent(_) | PostPentecost | PostEpiphany(_) | August | September | October | November => {
                    if day == Sun {
                        "sunday.gabc"
                    } else {
                        "feria.gabc"
                    }
                }
                Advent | Lent => {
                    if day == Sun {
                        "advent.gabc"
                    } else {
                        "feria.gabc"
                    }
                }
                _ => todo!("benedicamus for {:?}", iden.season),
            },
        ]
        .iter()
        .collect(),
    );

    Some(map)
}

pub fn vigils(iden: &Identifier) -> Option<HashMap<&'static str, PathBuf>> {
    let mut map: HashMap<&'static str, PathBuf> = HashMap::new();
    let day = iden.day.parse::<Weekday>().ok()?;
    match day {
        Sun => {
            map.insert(
                "absolution-1",
                "commons/vigils/1st-nocturn/absolution.gabc".into(),
            );
            map.insert(
                "blessing-1",
                "commons/vigils/1st-nocturn/blessing-1.gabc".into(),
            );
            map.insert(
                "blessing-2",
                "commons/vigils/1st-nocturn/blessing-2.gabc".into(),
            );
            map.insert(
                "blessing-3",
                "commons/vigils/1st-nocturn/blessing-3.gabc".into(),
            );

            map.insert(
                "absolution-2",
                "commons/vigils/2nd-nocturn/absolution.gabc".into(),
            );
            map.insert(
                "blessing-4",
                "commons/vigils/2nd-nocturn/blessing-1.gabc".into(),
            );
            map.insert(
                "blessing-5",
                "commons/vigils/2nd-nocturn/blessing-2.gabc".into(),
            );
            map.insert(
                "blessing-6",
                "commons/vigils/2nd-nocturn/blessing-3.gabc".into(),
            );

            map.insert(
                "absolution-3",
                "commons/vigils/3rd-nocturn/absolution.gabc".into(),
            );
            map.insert(
                "blessing-7",
                "commons/vigils/3rd-nocturn/blessing-1.gabc".into(),
            );
            map.insert(
                "blessing-8",
                "commons/vigils/3rd-nocturn/blessing-2.gabc".into(),
            );
            map.insert(
                "blessing-9",
                "commons/vigils/3rd-nocturn/blessing-3.gabc".into(),
            );
        }

        Mon | Thu => {
            map.insert(
                "absolution-1",
                "commons/vigils/1st-nocturn/absolution.gabc".into(),
            );
            map.insert(
                "blessing-1",
                "commons/vigils/1st-nocturn/blessing-1.gabc".into(),
            );
            map.insert(
                "blessing-2",
                "commons/vigils/1st-nocturn/blessing-2.gabc".into(),
            );
            map.insert(
                "blessing-3",
                "commons/vigils/1st-nocturn/blessing-3.gabc".into(),
            );
        }

        Tue | Fri => {
            map.insert(
                "absolution-1",
                "commons/vigils/2nd-nocturn/absolution.gabc".into(),
            );
            map.insert(
                "blessing-1",
                "commons/vigils/2nd-nocturn/blessing-1.gabc".into(),
            );
            map.insert(
                "blessing-2",
                "commons/vigils/2nd-nocturn/blessing-2.gabc".into(),
            );
            map.insert(
                "blessing-3",
                "commons/vigils/2nd-nocturn/blessing-3.gabc".into(),
            );
        }

        Wed | Sat => {
            map.insert(
                "absolution-1",
                "commons/vigils/3rd-nocturn/absolution.gabc".into(),
            );
            map.insert(
                "blessing-1",
                "commons/vigils/3rd-nocturn/blessing-1.gabc".into(),
            );
            map.insert(
                "blessing-2",
                "commons/vigils/3rd-nocturn/blessing-2.gabc".into(),
            );
            map.insert(
                "blessing-3",
                "commons/vigils/3rd-nocturn/blessing-3.gabc".into(),
            );
        }
    };

    for n in 1..=9 {
        map.insert(
            Box::leak(format!("lesson-{}", n).into_boxed_str()),
            iden.to_path().join("vigils").join("lessons").join(format!("{}.lit", n)),
        );
    }

    map.insert("gospel", iden.to_path().join("gospel.lit"));

    Some(map)
}
