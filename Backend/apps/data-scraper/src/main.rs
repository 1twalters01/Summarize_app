use dotenv::dotenv;
use clap::Parser;

mod args;
mod data_types;

use args::{ScrapeMode, ScraperArgs};

/* Normal mode
   - pass in an struct with Vec<String> params: {publishers, authors, books, genres}
   - (pass in the dataset)?
   - (Remove publishers already in dataset)?
   - Scrape publisher information
   - Get top 50 (non-fiction) authors for each publisher
   - Add scraped authors to the authors vec
   - (Remove authors already in dataset)?
   - Scrape author information
   - Get top 15 (non-fiction) books for each author
   - Add scraped books to the books vec
   - (Remove books already in dataset)?
   - Scrape info from books
   - Make a map of the genres
   - (Remove genres already in dataset)?
   - Return the publisher, author, book and genre information
*/
/* Initialise mode
   - Get top 20 (non-fiction) publishers from a site
   - Scrape publisher information
   - Get top 50 (non-fiction) authors for each publisher
   - Scrape author information
   - Get top 15 (non-fiction) books for each author
   - Scrape info from the books
   - Make a map of the genres
   - Return the publisher, author, book and genre information
*/

fn main() {
    dotenv().ok();

    let args = ScraperArgs::parse();
    match args.scrape_mode {
        ScrapeMode::Initialise => {
            initialise_process().await;
        },
        ScrapeMode::Continue => {
            continue_process().await;
        },
    }
}

#[cfg(test)]
mod tests {
}
