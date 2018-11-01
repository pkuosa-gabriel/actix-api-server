extern crate actix_web;
use actix_web::{server, App, HttpRequest};
use std::env; 

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

fn main() {
    let mut address = String::from("127.0.0.1:");
    match env::var("PORT") {
        Ok(port) => {
            println!("Port: {}", port);
            address.push_str(&port);
        },
        Err(e) => {
            println!("Couldn't read PORT ({}), will use default port: 8080", e);
            address.push_str("8080")
        }
    };
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind(address)
        .unwrap()
        .run();
}