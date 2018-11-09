extern crate diesel;
extern crate lycanthropy;

use self::lycanthropy::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop the newline character

    println!("Input your name:");
    let mut author = String::new();
    stdin().read_line(&mut author).unwrap();
    let author = &author[..(author.len() - 1)]; // Drop the newline character

    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        title, EOF
    );
    let mut contents = String::new();
    stdin().read_to_string(&mut contents).unwrap();

    let poem = post_poem(&connection, title, author, &contents);
    println!("\nSaved draft {} with id {}", title, poem.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
