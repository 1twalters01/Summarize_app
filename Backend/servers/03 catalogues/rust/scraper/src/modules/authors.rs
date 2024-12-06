use crate::{
    scraper,
    utils::validations::validate_author_vec
};
use std::{thread, time};

pub async fn run_authors_module(author_vec: Vec<String>) -> Result<(), Error> {
    // Validate author vec
    if let Err(err) = validate_author_vec(&author_vec) {
        return err
    }

    // Scrape author info
    let complete = false;
    let mut scraped_authors_name_vec: Vec<String> = Vec::new();
    let mut author_data_vec: Vec<Author> = Vec::new();
    while complete == false {
        let round_passed = true;

        author_vec.iter().map(|author_name| {
            if scraped_authors_name_vec.contains(author_name) == false {
                author_data_vec.push(scrape::author_information(author_name));
                scraped_authors_name.push(author_name)
                round_passed = false;
                let duration = time::Duration::from_millis(10);
                thread::sleep(duration);
            }
        })
        
        if round_passed = false { complete = true }
    };

    // format data

    // Save to postgres

    // Export from postgres to json
}
