use std::{
    collections::HashSet,
    io::{Error, ErrorKind},
    result::Result,
};

pub async fn run_formats_module(format_vec: Vec<String>) {
    // Validate format vec
    validate_format_vec(&format_vec).unwrap()

    // Scrape format info (format description)

    // format data

    // Save to postgres

    // Export from postgres to json
}


