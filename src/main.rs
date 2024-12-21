mod timehelp;
mod kalendar;
mod liturgy;
mod parser;
mod compiler;
mod router;
mod lexer;
mod git;

use clap::Parser;
use std::net::{TcpListener, SocketAddr};
use regex::Regex;
use std::collections::HashMap;
use std::path::PathBuf;

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


#[derive(Parser)]
#[command(author = "Daniel Write <daniel@writefamily.com>")]
#[command(version = "1.0")]
#[command(about = "Divine Office CLI")]
struct Cli {
    /// Optionally specify a port to launch the server on
    #[arg(short = 'l', long = "launch", value_name = "port", conflicts_with = "update")]
    launch: Option<String>,

    /// Update and save resources in the specified directory
    #[arg(short = 'u', long = "update", value_name = "resources save path", conflicts_with = "launch")]
    update: Option<PathBuf>,

    /// Specify where the resources are stored
    #[arg(long = "resources", value_name = "resources path", default_value = ".")]
    resources: PathBuf,
}

fn update(path: &PathBuf) -> Result<(), String> {
    use std::fs;

    println!("Cloning repository into {:?}", path);
    if path.exists() {
        println!("Directory exists, deleting (THIS WILL BE CHANGED SOMEDAY)");
        fs::remove_dir_all(path).map_err(|e| format!("Failed to delete existing directory: {}", e))?;
    }

    git::clone_repo(path)
}

fn main() {
     let cli = Cli::parse();

     if (cli.update.is_some()) {
        match update(&cli.update.unwrap()) {
            Ok(_) => println!("Successfully updated resources!"),
            Err(why) => {
                eprintln!("Failed to update resources: {}", why);
                std::process::exit(1);
            }
        }

        return; // do not start hosting
     }
    
    let port: u16 = match cli.launch {
        Some(port_str) => match port_str.parse() {
            Ok(p) => p,
            Err(_) => {
                eprintln!("Invalid port value. A random port will be assigned.");
                0
            }
        },
        None => 0
    };

    let bind_addr = TcpListener::bind(format!("0.0.0.0:{}", port))
        .expect("Failed to bind to a port")
        .local_addr()
        .expect("Failed to get local address");

    println!("Resources server from {:?}", cli.resources);
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
                router::handle_route(&cli.resources, id, params)
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
