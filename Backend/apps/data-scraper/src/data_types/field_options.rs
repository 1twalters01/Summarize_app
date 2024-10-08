use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Genre {
    pub genre: String,
    pub root: Option<Genre>,
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
    pub title: String,
    pub subtitle: String,
}

pub struct Publisher {
    pub publisher: String,
    pub root: Publisher,
}