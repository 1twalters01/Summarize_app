use std::{
    collections::HashSet,
    io::{Error, ErrorKind},
    result::Result,
};

pub async fn run_publishers_module(publisher_vec: Vec<String>) {
    // Validate puablisher vec
    validate_publisher_vec(&publisher_vec).unwrap()

    // while a complete pass hasn't been made
        // Scrape publisher info (subpublishers and superpublishers)
        // Add the subpublishers and superpublishers to publisher vec if not there already
        // If none are added then a pass is complete

    // replace super and sub publishers with ids

    // Save to postgres

    // Export to json
}
