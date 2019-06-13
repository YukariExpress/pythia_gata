use actix_web::{web, App, HttpServer, HttpRequest, Responder};
use std::env::var;
use std::net::{IpAddr, Ipv4Addr};

fn handler(r: HttpRequest) -> impl Responder {
    "ok"
}

fn main() -> std::io::Result<()> {

    let host: IpAddr = var("HOST").ok().and_then(|host| host.parse().ok())
        .unwrap_or_else(|| IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

    let port: u16 = var("PORT").ok().and_then(|port| port.parse().ok())
        .unwrap_or(8080);

    HttpServer::new(
        || App::new().service(
              web::resource("/").to(handler)))
        .bind((host, port))?
        .run()
}
