extern crate diesel;
extern crate lycanthropy;

use self::diesel::prelude::*;
use self::lycanthropy::*;
use self::models::*;

fn main() {
    use lycanthropy::schema::poems::dsl::*;

    let connection = establish_connection();
    let results = poems
        .filter(is_public.eq(true))
        .limit(5)
        .load::<Poem>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for poem in results {
        println!("{}", poem.title);
        println!("----------\n");
        println!("{}", poem.author);
        println!("----------\n");
        println!("{}", poem.contents);
    }
}
