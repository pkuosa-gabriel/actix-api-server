extern crate actix_web;
extern crate env_logger;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use actix_web::middleware::Logger;
use actix_web::{server, App, HttpRequest, Json, Result};
use std::env;

#[derive(Deserialize, Serialize)]
struct Poem {
    title: String,
    author: String,
    contents: String,
}

fn post_poem(mut poem: Json<Poem>) -> Result<Json<Poem>> {
    poem.title.push_str("_new");
    Ok(poem)
}

fn get_poems(_req: &HttpRequest) -> Result<Json<Poem>> {
    Ok(Json(Poem {
        title: String::from("Hello"),
        author: String::from("Mike"),
        contents: String::from("Hello, Mike!"),
    }))
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let mut address = String::from("127.0.0.1:");
    match env::var("PORT") {
        Ok(port) => {
            println!("Port: {}", port);
            address.push_str(&port);
        }
        Err(e) => {
            println!("Couldn't read PORT ({}), will use default port: 8080", e);
            address.push_str("8080")
        }
    };
    server::new(|| {
        App::new()
            .middleware(Logger::default())
            .middleware(Logger::new("%a %{User-Agent}i"))
            .prefix("/api/v1")
            .resource("/poems", |r| {
                r.get().f(get_poems);
                r.post().with(post_poem);
            })
    }).bind(address)
        .unwrap()
        .run();
}
