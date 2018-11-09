#[derive(Queryable)]
pub struct Poem {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub contents: String,
    pub is_public: bool,
}

use super::schema::poems;

#[derive(Insertable)]
#[table_name = "poems"]
pub struct NewPoem<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub contents: &'a str,
}
