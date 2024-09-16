mod timehelp;
mod kalendar;
mod liturgy;
mod parser;
mod compiler;
mod router;
mod lexer;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    // IP address and port to bind server to, e.g. localhost:80
    #[arg(short, long)]
    bind_to: String,
}

fn main() {
    use crate::router;

    let args = Args::parse();

    rouille::start_server(args.bind_to, move |request| {
        match request.url().as_str() {
            "/liturgy.css" => router::static_file("public/liturgy.css", "text/css"),
            "/suncalc.js" => router::static_file("public/suncalc.js", "application/javascript"),
            "/lit-time.js" => router::static_file("public/lit-time.js", "application/javascript"),
            "/exsurge.min.js" => router::static_file("public/exsurge.min.js", "application/javascript"),
            "/exsurge.min.js.map" => router::static_file("public/exsurge.min.js.map", "application/javascript"),
            "/" => router::static_file("public/index.html", "text/html"),
            _ => router::dynamic(request)
        }
    });
}