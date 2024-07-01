mod timehelp;
mod kalendar;
mod liturgy;
mod parser;
mod compiler;

use clap::Parser;

use serde::{Serialize};

use chrono::NaiveDate;


#[derive(Parser, Debug)]
struct Args {
    // IP address and port to bind server to, e.g. localhost:80
    #[arg(short, long)]
    ip_port: String,
}

fn main() {
    use rouille::Request;
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
                Response::text("{\"error\": \"An invalid date was supplied.\"}")
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

            // 404 anything else
            _ => {
                Response::empty_404()
            }
        )
    });
}

fn liturgy_info(date: NaiveDate) -> Result<LiturgyInfo, LiturgyError> {
    let kal = match crate::kalendar::Kalendar::from_date(date) {
        Some(kal) => kal,
        None => return Err(LiturgyError {
            error: format!("The supplied date {} is beyond the bounds of the Gregorian calendar.", date)
        })
    };

    let today = kal.get_celebrations(date);
    let tomorrow = kal.get_celebrations(date + chrono::Days::new(1));

    let today = &today[0];
    let tomorrow = &tomorrow[0];

    let liturgy = crate::liturgy::resolve_hours(today, tomorrow);

    Ok(LiturgyInfo {
        vigils: today.clone(),
        vespers: if liturgy.today_vespers.unwrap() { None } else { Some(tomorrow.clone()) }
    })
}

#[derive(Serialize)]
struct LiturgyError {
    error: String
}

#[derive(Serialize)]
struct LiturgyInfo {
    vigils: crate::kalendar::Celebration,
    vespers: Option<crate::kalendar::Celebration>
}