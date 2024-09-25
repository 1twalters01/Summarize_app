use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct GenreField {
    pub genre: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthorField {
    pub uuid: Uuid,
    pub first_name: String,
    pub last_name: Option<String>,
    pub middle_names: Vec<String>,
    pub genres: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BookField {
    pub title: String,
    pub subtitle: String,
}

