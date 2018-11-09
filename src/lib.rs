pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

use self::models::{NewPoem, Poem};

pub fn post_poem<'a>(
    conn: &PgConnection,
    title: &'a str,
    author: &'a str,
    contents: &'a str,
) -> Poem {
    use schema::poems;

    let new_poem = NewPoem {
        title: title,
        author: author,
        contents: contents,
    };

    diesel::insert_into(poems::table)
        .values(&new_poem)
        .get_result(conn)
        .expect("Error saving new poem")
}
