use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct Publisher {
    pub publisher: String,
    pub root: Publisher,
}

#[derive(Serialize, Deserialize)]
pub struct Author {
    pub uuid: Uuid,
    pub first_name: String,
    pub last_name: Option<String>,
    pub middle_names: Vec<String>,
    pub genres: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub title: Vec<String>,
    pub languages: Vec<String>,
    pub subtitle: Vec<String>,
    pub isbn: i64,
    pub isbn_13: i64,
    pub authors: Vec<Author>,
    pub co_authors: Vec<Author>,
    pub publisher: Publisher,
    pub edition: String,
    pub publish_date: SystemTime,
    pub binding: String,
    pub pages: i64,
    pub synopsis: String,
    pub dimensions: Dimensions, // height, length, width
    pub weight: Weight,
    pub genres: Vec<Genre>,
    pub ratings: Ratings,
    pub links: Links, 
}

#[derive(Serialize, Deserialize)]
pub struct Genre {
    pub genre: String,
    pub supergenre: Option<Genre>,
    pub subgenres: Vec<Option<Genre>>,
}