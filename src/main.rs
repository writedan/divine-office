mod timehelp;
mod kalendar;
mod liturgy;
mod parser;
mod compiler;

use clap::Parser;

use serde::{Serialize};

use chrono::NaiveDate;

use std::collections::HashMap;

use std::path::PathBuf;

use crate::kalendar::Celebration;


#[derive(Parser, Debug)]
struct Args {
    // IP address and port to bind server to, e.g. localhost:80
    #[arg(short, long)]
    ip_port: String,
}

fn main() {
    
    use rouille::Response;
    use rouille::router;

    use std::fs::File;

    let args = Args::parse();

    rouille::start_server(args.ip_port, move |request| {
        router!(request,
            // frontends for the end user
            (GET) ["/"] => {
                let file = File::open("public/index.html").unwrap();
                Response::from_file("text/html", file)
            },

            (GET) ["/{y}/{m}/{d}/{h}", y: i32, m: u32, d: u32, h: String] => {
                let date = match NaiveDate::from_ymd_opt(y, m, d) {
                    Some(date) => date,
                    None => return Response::json(&LiturgyError {
                        error: format!("An invalid date y={} m={} d={} was supplied.", y, m, d)
                    })
                };

                let (today, tomorrow) = match get_liturgies(date) {
                    Ok(lit) => lit,
                    Err(why) => return Response::json(&why)
                };

                let lit = crate::liturgy::resolve_hours(&today, &tomorrow);

                let h = h.as_str();
                match h {
                    "vigils" => compile_hour(lit.vigils),
                    "matins" => compile_hour(lit.matins),
                    "prime" => compile_hour(lit.prime),
                    "terce" => compile_hour(lit.terce),
                    "sext" => compile_hour(lit.sext),
                    "none" => compile_hour(lit.none),
                    "vespers" => compile_hour(lit.vespers),
                    "compline" => compile_hour(lit.compline),
                    _ => return Response::json(&LiturgyError {
                        error: format!("An invalid hour \"{}\" was supplied.", h)
                    })
                }
            },

            // api endpoints
            (GET) ["/api/{y}/{m}/{d}", y: i32, m: u32, d: u32] => {
                let date = match NaiveDate::from_ymd_opt(y, m, d) {
                    Some(date) => date,
                    None => return Response::json(&LiturgyError {
                        error: format!("An invalid date y={} m={} d={} was supplied.", y, m, d)
                    })
                };

                match liturgy_info(date) {
                    Ok(info) => Response::json(&info),
                    Err(err) => Response::json(&err)
                }
            },

            // static pages
            (GET) ["/liturgy.css"] => {
                let file = File::open("public/liturgy.css").unwrap();
                Response::from_file("text/css", file)
            },

            (GET) ["/suncalc.js"] => {
                let file = File::open("public/suncalc.js").unwrap();
                Response::from_file("application/javascript", file)
            },

            (GET) ["/lit-time.js"] => {
                let file = File::open("public/lit-time.js").unwrap();
                Response::from_file("application/javascript", file)
            },

            (GET) ["/exsurge.min.js"] => {
                let file = File::open("public/exsurge.min.js").unwrap();
                Response::from_file("application/javascript", file)
            },

            (GET) ["/exsurge.min.js.map"] => {
                let file = File::open("public/exsurge.min.js.map").unwrap();
                Response::from_file("application/javascript", file)
            },

            // 404 anything else
            _ => {
                Response::empty_404()
            }
        )
    });
}

fn compile_hour(propers: HashMap<&'static str, PathBuf>) ->rouille::Response {
    use build_html::Html;
    use std::fs;

    let elements = crate::compiler::compile_ast(crate::parser::parse_hour(propers));
    let mut buf = String::new();
    for ele in elements {
        buf.push_str(&ele.to_html_string());
    }

    let mut base: String = fs::read_to_string("public/liturgy.html").unwrap();

    base = base.replace("<%= content %>", &buf);
    
    rouille::Response::html(base)
}

fn get_liturgies(date: NaiveDate) -> Result<(Celebration, Celebration), LiturgyError> {
    let today = match crate::kalendar::get_celebration(date) {
        Some(kal) => kal,
        None => return Err(LiturgyError {
            error: format!("The supplied date {} is beyond the bounds of the Gregorian calendar.", date)
        })
    };

    let tomorrow = match crate::kalendar::get_celebration(date + chrono::Days::new(1)) {
        Some(kal) => kal,
        None => return Err(LiturgyError {
            error: format!("The supplied date {} is beyond the bounds of the Gregorian calendar.", date + chrono::Days::new(1))
        })
    };

    Ok((today, tomorrow))
}

fn liturgy_info(date: NaiveDate) -> Result<LiturgyInfo, LiturgyError> {
    let (today, tomorrow) = get_liturgies(date)?;
    let today_vespers = !crate::liturgy::first_vespers(&today, &tomorrow);

    Ok(LiturgyInfo {
        today: today.clone(),
        tomorrow: if today_vespers { None } else { Some(tomorrow.clone()) },
    })
}

#[derive(Serialize)]
struct LiturgyError {
    error: String
}

#[derive(Serialize)]
struct LiturgyInfo {
    today: crate::kalendar::Celebration,
    tomorrow: Option<crate::kalendar::Celebration>
}