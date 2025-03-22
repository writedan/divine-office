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
    HashMap::new()
}

fn vigils(iden: &Identifier) -> HashMap<&'static str, PathBuf> {
    HashMap::new()
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
