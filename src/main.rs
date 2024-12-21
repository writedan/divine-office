mod timehelp;
mod kalendar;
mod liturgy;
mod parser;
mod compiler;
mod router;
mod lexer;

use clap::{Command, Arg};
use std::net::{TcpListener, SocketAddr};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Route {
    url_pattern: String,
}

struct Router {
    routes: HashMap<String, Route>,
}

impl Router {
    fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    fn add_route(&mut self, id: &str, url_pattern: &str) {
        let route = Route {
            url_pattern: url_pattern.to_string(),
        };
        self.routes.insert(id.to_string(), route);
    }

    fn get_route_id(&self, url: &str) -> Option<(String, HashMap<String, String>)> {
        for (id, route) in &self.routes {
            let regex_pattern = self.create_regex_pattern(&route.url_pattern);

            let regex = Regex::new(&regex_pattern).unwrap();

            if let Some(captures) = regex.captures(url) {
                let mut params = HashMap::new();
                let param_names = self.extract_param_names(&route.url_pattern);

                for (i, param_name) in param_names.iter().enumerate() {
                    let value = captures.get(i + 1).unwrap().as_str().to_string();
                    params.insert(param_name.clone(), value);
                }

                return Some((id.clone(), params));
            }
        }

        None
    }

    fn create_regex_pattern(&self, url_pattern: &str) -> String {
        let re = Regex::new(r"\{(\w+):(\w+)\}").unwrap();
        re.replace_all(url_pattern, |caps: &regex::Captures| {
            let param_type = &caps[2];
            match param_type {
                "integer" => r"(\d+)".to_string(),
                "string" => r"([^/]+)".to_string(),
                _ => r"([^/]+)".to_string(),
            }
        }).to_string()
    }

    fn extract_param_names(&self, url_pattern: &str) -> Vec<String> {
        let re = Regex::new(r"\{(\w+):\w+\}").unwrap();
        re.captures_iter(url_pattern)
            .map(|caps| caps[1].to_string())
            .collect()
    }
}

fn main() {
     let matches = Command::new("Divine Office")
        .version("1.0")
        .author("Daniel Write <daniel@writefamily.com>")
        .arg(
            Arg::new("launch")
                .short('l')
                .long("launch")
                .value_name("PORT")
                .help("Optionally specify a port to launch the server on")
        )
        .get_matches();

    let binding = "0".to_string();
    let port = matches.get_one::<String>("launch").unwrap_or(&binding);

    let port: u16 = match port.parse() {
        Ok(p) => p,
        Err(_) => {
            eprintln!("Invalid port value. A random port will be assigned.");
            0
        }
    };

    let bind_addr = TcpListener::bind(format!("0.0.0.0:{}", port))
        .expect("Failed to bind to a port")
        .local_addr()
        .expect("Failed to get local address");

    println!("Server running at http://127.0.0.1:{}", bind_addr.port());

    let mut router = Router::new();

    router.add_route("LiturgicalIdentifier", "/Identifiers/Day/{year:integer}-{month:integer}-{day:integer}");
    router.add_route("MonthlyLiturgicalIdentifiers", "/Identifiers/Month/{year:integer}-{month:integer}");
    router.add_route("HourCompiledElements", "/Elements/{year:integer}-{month:integer}-{day:integer}/{hour:string}");

    rouille::start_server(bind_addr.to_string(), move |request| {
        use crate::router;

        if request.method() == "OPTIONS" {
            return rouille::Response::text("")
                .with_status_code(204)
                .with_additional_header("Access-Control-Allow-Origin", "*")
                .with_additional_header("Access-Control-Allow-Methods", "GET, OPTIONS")
                .with_additional_header("Access-Control-Allow-Headers", "*");
        }

        let mut response = match router.get_route_id(request.url().as_str()) {
            Some((id, params)) => {
                router::handle_route(id, params)
            },
            None => rouille::Response::empty_404(),
        };

        response = response
            .with_additional_header("Access-Control-Allow-Origin", "*")
            .with_additional_header("Access-Control-Allow-Methods", "GET, OPTIONS")
            .with_additional_header("Access-Control-Allow-Headers", "*");

        return response;
    });
}
