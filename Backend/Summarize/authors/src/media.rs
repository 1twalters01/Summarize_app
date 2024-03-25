use crate::author::Author;

pub struct Media {
    id: String,
    title: String,
    subtitle: String,
    images: Vec<String>,
    authors: Vec<Author>,
    contributors: Vec<String>,
    description: String,
    url: String,
    media_type: String,
}
