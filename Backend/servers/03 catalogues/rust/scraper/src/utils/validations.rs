use std::{
    collections::HashSet,
    io::{Error, ErrorKind},
    result::Result,
};

pub fn validate_format_vec(format_vec: &Vec<String>) -> Result<(), Error> {
    let format_hs: HashSet<String> = format_vec.iter().cloned().collect::<HashSet<String>>();

    if format_hs.len() != format_vec.len() {
        let error: Error = Error::new(
            ErrorKind::InvalidData,
            format!("Format vector has duplicated formats"),
        );
        return Err(error);
    }

    if format_vec.contains(&String::new()) {
        let error: Error = Error::new(
            ErrorKind::InvalidData,
            format!("format vector has null element(s)"),
        );
        return Err(error);
    }
}

pub fn validate_author_vec(author_vec: &Vec<String>) -> Result<(), Error> {
    let author_hs: HashSet<String> = author_vec.iter().cloned().collect::<HashSet<String>>();

    if author_hs.len() != author_vec.len() {
        let error: Error = Error::new(
            ErrorKind::InvalidData,
            format!("Author vector has duplicated authors"),
        );
        return Err(error);
    }

    if author_vec.contains(&String::new()) {
        let error: Error = Error::new(
            ErrorKind::InvalidData,
            format!("Author vector has null element(s)"),
        );
        return Err(error);
    }
}

pub fn validate_genre_vec(genre_vec: &Vec<String>) -> Result<(), Error> {
    let genre_hs: HashSet<String> = genre_vec.iter().cloned().collect::<HashSet<String>>();

    if genre_hs.len() != genre_vec.len() {
        let error: Error = Error::new(
            ErrorKind::InvalidData,
            format!("Genre vector has duplicated genres"),
        );
        return Err(error);
    }

    if genre_vec.contains(&String::new()) {
        let error: Error = Error::new(
            ErrorKind::InvalidData,
            format!("Genre vector has null element(s)"),
        );
        return Err(error);
    }
}

pub fn validate_publisher_vec(publisher_vec: &Vec<String>) -> Result<(), Error> {
    let publisher_hs: HashSet<String> = publisher_vec.iter().cloned().collect::<HashSet<String>>();

    if publisher_hs.len() != publisher_vec.len() {
        let error: Error = Error::new(
            ErrorKind::InvalidData,
            format!("Publisher vector has duplicated publishers"),
        );
        return Err(error);
    }

    if publisher_vec.contains(&String::new()) {
        let error: Error = Error::new(
            ErrorKind::InvalidData,
            format!("Publisher vector has null element(s)"),
        );
        return Err(error);
    }
}