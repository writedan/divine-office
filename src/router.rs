use rouille::{Request, Response};
use serde::Serialize;
use std::path::PathBuf;
use std::collections::HashMap;
use chrono::NaiveDate;
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
                    Some(date) => Ok(Response::json(&get_identifiers(date))),
                    None => Err(format!("Unable to parse date {}-{}-{}", y, m, d))
                }
            } else {
                Err(format!("Unable to parse paramters: {:?}", params))
            }
        },

        &_ => Err(format!("Unknown ID {}: {:?}", id, params))
    }
}

fn get_identifiers(date: NaiveDate) -> Result<LiturgyInfo, LiturgyError> {
    let today = match kalendar::get_celebration(date) {
        Some(kal) => kal,
        None => return Err(LiturgyError {
            error: format!("The supplied date {} is beyond the bounds of the Gregorian calendar.", date)
        })
    };

    let tomorrow = match kalendar::get_celebration(date + chrono::Days::new(1)) {
        Some(kal) => kal,
        None => return Err(LiturgyError {
            error: format!("The supplied date {} is beyond the bounds of the Gregorian calendar.", date + chrono::Days::new(1))
        })
    };

    let today_vespers = !liturgy::first_vespers(&today, &tomorrow);

    Ok(LiturgyInfo {
        today: today,
        tomorrow: if today_vespers { None } else { Some(tomorrow) },
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