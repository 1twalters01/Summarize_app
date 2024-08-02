use chronos::time;
use uuid::Uuid;

pub struct Book {
    information: Information,
    details: Details,
    synopsis: Synopsis,
    
    related_works: Related,
}

pub struct Information {
    uuid: UUID,
    title: String,
    image: Bytes,
    subtitle: String,
    authors: Vec<Author>,
    contributors: Vec<Author>,
    publication_date: time,
    publisher: Publisher,
}

pub struct Details {
    isbn: ISBN,
    original_languages: Vec<String>,
    translations: Vec<String>,
    editions: Vec<String>,
    formats: Vec<Format>,
    page_count: u64,
    original_language: String,
    cover_image: String, //link
    publisher: Publisher,
    blurb: String,
}

pub enum Format {
    Hardcover,
    Paperback,
    ebook,
    audiobook,
}

pub struct Synopsis {
    summary: String,
    genre: Genre,
    themes: Vec<String>,
}

pub struct Related {
    prequels: Vec<Book>,
    sequels: Vec<Book>,
    other: Vec<Book>,
    adaptations: Vec<Adaptation>,
}

pub struct Adaptation {
    type: AdaptationType, // Films, tv shows, plays, etc.
    link: String,
}

pub enum AdaptationType {
    Film,
    TVShow,
    Play,
}

