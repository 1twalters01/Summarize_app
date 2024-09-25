use dotenv::dotenv;
use clap::Parser;

mod args;
mod data_types;

use args::{ScrapeMode, ScraperArgs};

// Normal mode - pass in an struct/array? of artists and books, scrape info and top 30 books from the artists and add the books to book list, then scrape all necessary info from books
// Initialise mode - get top 30 genres, scrape each for the top 20 artists. get info from each and the top 30 books from each. Scrape all info from the books.

fn main() {
    dotenv().ok();

    let args = ScraperArgs::parse();
    match args.scrape_mode {
        ScrapeMode::Initialise => {
        },
        ScrapeMode::Continue => {
        },
    }
}

#[cfg(test)]
mod tests {
}
