use crate::kalendar::Identifier;
use crate::liturgy::Liturgy;

use crate::kalendar::Season::*;

use std::collections::HashMap;
use std::path::PathBuf;

mod passion;

pub fn resolve(iden: &Identifier) -> Liturgy {
    match iden.week.as_str() {
        "6" => passion::resolve(iden),
        _ => todo!("Get parts for Lent week {}", iden.week)
    }
}