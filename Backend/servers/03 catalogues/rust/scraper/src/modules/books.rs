use std::{
    collections::HashSet,
    io::{Error, ErrorKind},
    result::Result,
};

pub async fn run_books_module(author_vec: Vec<String>) -> Result<(), Error> {
    // Validate book vec
    validate_author_vec(&author_vec).unwrap()

    /* Scrape book info
    book title, subtitle and edition
    book formats
    authors
    co-authors
    genres
    publisher
    pages
    isbn
    isbn13
    synopsis
    links to buy (amazon, etc.)
    */
    
    // check if all formats have been scraped else scraped them
    // check if all authors and coauthors have been scraped else scrape them
    // check if all genres have been scraped else scrape them
    // check if all publishers have been scraped else scrape them

    // Get ids for all formats
    // get ids for all genres
    // get ids for all publishers
    // get ids for all authors and coauthors

    // create book struct

    // Save to postgres

    // Export from postgres to json
}

pub fn validate_books_vec(author_vec: &Vec<String>) -> Result<(), Error> {
    let book_hs: HashSet<String> = book_vec.iter().cloned().collect::<HashSet<String>>();

    if book_hs.len() != book_vec.len() {
        let error: Error = Error::new(
            ErrorKind::InvalidData,
            format!("Book vector has duplicated books"),
        );
        return Err(error);
    }

    if author_vec.contains(&String::new()) {
        let error: Error = Error::new(
            ErrorKind::InvalidData,
            format!("Book vector has null element(s)"),
        );
        return Err(error);
    }
}