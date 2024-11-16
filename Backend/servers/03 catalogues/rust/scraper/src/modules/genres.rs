use std::{
    collections::HashSet,
    io::{Error, ErrorKind},
    result::Result,
};

pub async fn run_genres_module(genre_vec: Vec<String>) {
    // Validate genre vec
    validate_genre_vec(&genre_vec).unwrap()

    // Scrape genre info (subgenres and supergenres)
    // while a complete pass hasn't been made
        // Scrape publisher info (subgenres and supergenres)
        // Add the subgenres and supergenres to publisher vec if not there already
        // If none are added then a pass is complete

    // replace super and sub genres with ids

    // format data

    // Save to postgres

    // Export from postgres to json
}