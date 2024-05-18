pub struct Book {
    id: UUID,
    title: String,
    subtitle: String,
    isbn: ISBN,
    authors: Vec<Author>,
    publisher: Publisher,
    blurb: String,
    editions: Vec<Book>,
    ratings: Vec<Ratings>,
    Media: Vec<Media>,
    Summaries: Vec<Summaries>,
}

pub struct Publisher {
}
