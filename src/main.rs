mod timehelp;
mod kalendar;
mod liturgy;
mod parser;
mod compiler;
mod router;
mod lexer;

use std::net::{TcpListener, SocketAddr};

fn main() {
    use crate::router;

    let bind_addr = TcpListener::bind("0.0.0.0:0")
        .expect("Failed to bind to a port")
        .local_addr()
        .expect("Failed to get local address");

    println!("http://127.0.0.1:{}", bind_addr.port());

    rouille::start_server(bind_addr.to_string(), move |request| {
        if request.method() == "OPTIONS" {
            return rouille::Response::text("")
                .with_status_code(204)
                .with_additional_header("Access-Control-Allow-Origin", "*")
                .with_additional_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
                .with_additional_header("Access-Control-Allow-Headers", "*");
        }

        let mut response = match request.url().as_str() {
            "/liturgy.css" => router::static_file("public/liturgy.css", "text/css"),
            "/suncalc.js" => router::static_file("public/suncalc.js", "application/javascript"),
            "/lit-time.js" => router::static_file("public/lit-time.js", "application/javascript"),
            "/exsurge.min.js" => router::static_file("public/exsurge.min.js", "application/javascript"),
            "/exsurge.min.js.map" => router::static_file("public/exsurge.min.js.map", "application/javascript"),
            "/" => router::static_file("public/index.html", "text/html"),
            _ => router::dynamic(request),
        };

        response = response
            .with_additional_header("Access-Control-Allow-Origin", "*")
            .with_additional_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
            .with_additional_header("Access-Control-Allow-Headers", "*");

        response
    });
}
