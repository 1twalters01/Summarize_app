use crate::utils::validations::validate_author_vec;

pub async fn run_authors_module(author_vec: Vec<String>) -> Result<(), Error> {
    // Validate author vec
    if let Err(err) = validate_author_vec(&author_vec) {
        return err
    }

    /* Scrape author info
    first_name
    last_name
    middle_name
    pen_name
    date_of_birth
    date_of_death
    */

    // format data

    // Save to postgres

    // Export from postgres to json
}
