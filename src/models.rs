#[derive(Queryable)]
pub struct Poem {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub contents: String,
    pub is_public: bool,
}