use rouille::{Request, Response};
use serde::Serialize;
use std::path::PathBuf;
use std::collections::HashMap;
use chrono::{NaiveDate, Datelike};
use crate::kalendar::Celebration;
use crate::{kalendar, liturgy, compiler, parser};
use std::fs::File;

type R<T> = Result<T, String>;

pub fn handle_route(id: String, params: HashMap<String, String>) -> Response {
    match route(id, params) {
        Ok(resp) => resp,
        Err(error) => Response::json(&LiturgyError {
            error
        })
    }
}

fn route(id: String, params: HashMap<String, String>) -> R<Response> {
    match (id.as_str()) {
        "LiturgicalIdentifier" => {
            if let (Ok(y), Ok(m), Ok(d)) = (params["year"].parse(), params["month"].parse(), params["day"].parse()) {
                match NaiveDate::from_ymd_opt(y, m, d) {
                    Some(date) => Ok(Response::json(&liturgy_info(date)?)),
                    None => Err(format!("Unable to parse date {}-{}-{}", y, m, d))
                }
            } else {
                Err(format!("Unable to parse paramters: {:?}", params))
            }
        },

        "MonthlyLiturgicalIdentifiers" => {
            if let (Ok(y), Ok(m)) = (params["year"].parse(), params["month"].parse()) {
                let first_day_of_month = NaiveDate::from_ymd(y, m, 1);
                let next_month = first_day_of_month
                    .with_month((m + 1))
                    .unwrap_or_else(|| NaiveDate::from_ymd(y + 1, 1, 1));
                let days_in_month = (next_month - first_day_of_month).num_days();

                let mut month: HashMap<u32, Celebration> = HashMap::new();

                println!("there are {} days in month {}", days_in_month, m);
                println!("next month is {:?}", next_month);

                for day in 1..=days_in_month {
                    let date = NaiveDate::from_ymd(y, m, day as u32);

                    let identifier = get_identifiers(date)?.0;
                    month.insert(day as u32, identifier);
                }

                Ok(rouille::Response::json(&month))
            } else {
                Err(format!("Unable to parse paramters: {:?}", params))
            }
        },

        "HourCompiledElements" => {
            if let (Ok(y), Ok(m), Ok(d)) = (params["year"].parse(), params["month"].parse(), params["day"].parse()) {
                match NaiveDate::from_ymd_opt(y, m, d) {
                    Some(date) => {
                        let (today, tomorrow) = match get_identifiers(date) {
                            Ok(lit) => lit,
                            Err(why) => return Err(why)
                        };

                        let lit = liturgy::resolve_hours(&today, &tomorrow);
                        match params["hour"].as_str() {
                            "vigils" => Ok(compile_hour(lit.vigils)),
                            "matins" => Ok(compile_hour(lit.matins)),
                            "prime" => Ok(compile_hour(lit.prime)),
                            "terce" => Ok(compile_hour(lit.terce)),
                            "sext" => Ok(compile_hour(lit.sext)),
                            "none" => Ok(compile_hour(lit.none)),
                            "vespers" => Ok(compile_hour(lit.vespers)),
                            "compline" => Ok(compile_hour(lit.compline)),
                            _ => Err(format!("An invalid hour \"{}\" was supplied.", params["hour"]))
                        }
                    },
                    None => Err(format!("Unable to parse date {}-{}-{}", y, m, d))
                }
            } else {
                Err(format!("Unable to parse paramters: {:?}", params))
            }
        }

        &_ => Err(format!("Unknown ID {}: {:?}", id, params))
    }
}

fn compile_hour(propers: HashMap<&'static str, PathBuf>) -> Response {
    Response::json(&compiler::compile_ast(parser::Parser::from_hour(propers)))
}

fn get_identifiers(date: NaiveDate) -> R<(Celebration, Celebration)> {
    let today = match kalendar::get_celebration(date) {
        Some(kal) => kal,
        None => return Err(format!("The supplied date {} is beyond the bounds of the Gregorian calendar.", date))
    };

    let tomorrow = match kalendar::get_celebration(date + chrono::Days::new(1)) {
        Some(kal) => kal,
        None => return Err(format!("The supplied date {} is beyond the bounds of the Gregorian calendar.", date + chrono::Days::new(1)))
    };

    let today_vespers = !liturgy::first_vespers(&today, &tomorrow);

    Ok((today, tomorrow))
}

fn liturgy_info(date: NaiveDate) -> R<(LiturgyInfo)> {
    let (today, tomorrow) = get_identifiers(date)?;
    let today_vespers = !liturgy::first_vespers(&today, &tomorrow);
    
    Ok(LiturgyInfo {
        today,
        tomorrow: if today_vespers { None } else { Some(tomorrow) }
    })
}

#[derive(Serialize)]
struct LiturgyError {
    error: String
}

#[derive(Serialize)]
struct LiturgyInfo {
    today: Celebration,
    tomorrow: Option<Celebration>
}