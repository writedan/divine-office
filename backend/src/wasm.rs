use crate::{compiler, kalendar, liturgy, parser};
use chrono::{Datelike, NaiveDate};
use std::collections::HashMap;

type R<T> = Result<T, LiturgyError>;

impl From<String> for LiturgyError {
    fn from(error: String) -> Self {
        LiturgyError { error }
    }
}

fn from_ymd(y: i32, m: u32, d: u32) -> R<NaiveDate> {
    match NaiveDate::from_ymd_opt(y, m, d) {
        Some(date) => Ok(date),
        None => Err(format!("Invalid date: {}-{}-{}.", y, m, d).into()),
    }
}

fn get_identifiers(date: NaiveDate) -> R<(kalendar::Celebration, kalendar::Celebration)> {
    let today = match kalendar::get_celebration(date) {
        Some(kal) => kal,
        None => {
            return Err(format!(
                "The supplied date {} is beyond the bounds of the Gregorian calendar.",
                date
            )
            .into())
        }
    };

    let tomorrow = match kalendar::get_celebration(date + chrono::Days::new(1)) {
        Some(kal) => kal,
        None => {
            return Err(format!(
                "The supplied date {} is beyond the bounds of the Gregorian calendar.",
                date + chrono::Days::new(1)
            )
            .into())
        }
    };

    let today_vespers = !liturgy::first_vespers(&today, &tomorrow);

    Ok((today, tomorrow))
}

fn compile_hour(propers: HashMap<&'static str, std::path::PathBuf>) -> Vec<compiler::Element> {
    compiler::compile_ast(parser::Parser::from_hour(propers))
}

pub fn get_identifier(y: i32, m: u32, d: u32) -> R<LiturgyInfo> {
    let date = from_ymd(y, m, d)?;

    let (today, tomorrow) = get_identifiers(date)?;
    let today_vespers = !liturgy::first_vespers(&today, &tomorrow);

    Ok(LiturgyInfo {
        today,
        tomorrow: if today_vespers { None } else { Some(tomorrow) },
    })
}

pub fn get_monthly_identifiers(y: i32, m: u32) -> R<HashMap<u32, kalendar::Celebration>> {
    let first_day_of_month = from_ymd(y, m, 1)?;
    let next_month = first_day_of_month
        .with_month(m + 1)
        .unwrap_or_else(|| NaiveDate::from_ymd(y + 1, 1, 1));
    let days_in_month = (next_month - first_day_of_month).num_days();

    let mut month: HashMap<u32, kalendar::Celebration> = HashMap::new();

    for day in 1..=days_in_month {
        let date = from_ymd(y, m, day as u32)?;

        let identifier = get_identifiers(date)?.0;
        month.insert(day as u32, identifier);
    }

    Ok(month)
}

pub fn get_hour(y: i32, m: u32, d: u32, hour: &str) -> R<Vec<compiler::Element>> {
    let (today, tomorrow) = match get_identifiers(from_ymd(y, m, d)?) {
        Ok(lit) => lit,
        Err(why) => return Err(why),
    };

    let lit = liturgy::resolve_hours(&today, &tomorrow);

    match hour {
        "vigils" => Ok(compile_hour(lit.vigils)),
        "matins" => Ok(compile_hour(lit.matins)),
        "prime" => Ok(compile_hour(lit.prime)),
        "terce" => Ok(compile_hour(lit.terce)),
        "sext" => Ok(compile_hour(lit.sext)),
        "none" => Ok(compile_hour(lit.none)),
        "vespers" => Ok(compile_hour(lit.vespers)),
        "compline" => Ok(compile_hour(lit.compline)),
        _ => Err(format!("An invalid hour \"{}\" was supplied.", hour).into()),
    }
}

#[derive(serde::Serialize)]
pub struct LiturgyError {
    error: String,
}

#[derive(serde::Serialize)]
pub struct LiturgyInfo {
    today: kalendar::Celebration,
    tomorrow: Option<kalendar::Celebration>,
}
