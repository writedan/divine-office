use rouille::{Request, Response};
use serde::Serialize;
use std::path::PathBuf;
use std::collections::HashMap;
use chrono::NaiveDate;
use crate::kalendar::Celebration;
use crate::{kalendar, liturgy, compiler, parser};
use std::fs::File;

type R<T> = Result<T, String>;

pub fn static_file(path: &str, mime: &str) -> Response {
	match File::open(path) {
		Ok(path) => Response::from_file(mime.to_owned(), path),
		Err(why) => Response::json(&LiturgyError {
			error: format!("Failed to open \"{}\": {}", path, why)
		})
	}
}

pub fn dynamic(req: &Request) -> Response {
	match handle_dynamic(req) {
		Ok(resp) => resp,
		Err(error) => Response::json(&LiturgyError {
			error
		})
	}
}

fn handle_dynamic(req: &Request) -> R<Response> {
	let url = req.url();
    let path_parts: Vec<&str> = url.trim_start_matches('/').split('/').collect();

    if path_parts.len() == 2 {
    	let (date, hour) = extract_date_and_hour(path_parts[0], path_parts[1])?;

    	let (today, tomorrow) = match get_liturgies(date) {
    		Ok(lit) => lit,
    		Err(why) => return Err(why.error)
    	};

    	let lit = liturgy::resolve_hours(&today, &tomorrow);

    	match hour.as_str() {
    		"vigils" => Ok(compile_hour(lit.vigils)),
    		"matins" => Ok(compile_hour(lit.matins)),
    		"prime" => Ok(compile_hour(lit.prime)),
    		"terce" => Ok(compile_hour(lit.terce)),
    		"sext" => Ok(compile_hour(lit.sext)),
    		"none" => Ok(compile_hour(lit.none)),
    		"vespers" => Ok(compile_hour(lit.vespers)),
    		"compline" => Ok(compile_hour(lit.compline)),
    		_ => Err(format!("An invalid hour \"{}\" was supplied.", hour))
    	}
    } else if url.starts_with("/api") {
    	let date = extract_date(path_parts[1])?;
    	match liturgy_info(date) {
    		Ok(info) => Ok(Response::json(&info)),
    		Err(err) => Ok(Response::json(&err))
    	}
    } else {
    	Ok(Response::empty_404())
	}
}

fn extract_date(date_part: &str) -> R<NaiveDate> {
    let date_parts: Vec<&str> = date_part.split('-').collect();
    if date_parts.len() == 3 {
        if let (Ok(y), Ok(m), Ok(d)) = (date_parts[0].parse(), date_parts[1].parse(), date_parts[2].parse()) {
            return match NaiveDate::from_ymd_opt(y, m, d) {
            	Some(date) => Ok(date),
            	None => Err(format!("Unable to parse date {}-{}-{}", y, m, d))
            };
        }
    }

    Err(format!("Unable to parse raw date {}", date_part))
}

fn extract_date_and_hour(date_part: &str, hour: &str) -> R<(NaiveDate, String)> {
	let date = extract_date(date_part)?;
	Ok((date, hour.to_string()))
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

fn get_liturgies(date: NaiveDate) -> Result<(Celebration, Celebration), LiturgyError> {
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

    Ok((today, tomorrow))
}

fn liturgy_info(date: NaiveDate) -> Result<LiturgyInfo, LiturgyError> {
    let (today, tomorrow) = get_liturgies(date)?;
    let today_vespers = !liturgy::first_vespers(&today, &tomorrow);

    Ok(LiturgyInfo {
        today: today.clone(),
        tomorrow: if today_vespers { None } else { Some(tomorrow.clone()) },
    })
}

fn compile_hour(propers: HashMap<&'static str, PathBuf>) ->Response {
    use build_html::Html;
    use std::fs;

    let elements = compiler::compile_ast(parser::parse_hour(propers));
    let mut buf = String::new();
    for ele in elements {
        buf.push_str(&ele.to_html_string());
    }

    let mut base: String = fs::read_to_string("public/liturgy.html").unwrap();

    base = base.replace("<%= content %>", &buf);
    
    Response::html(base)
}
