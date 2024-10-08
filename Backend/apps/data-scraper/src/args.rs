use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct ScraperArgs {
    /// Scrape mode
    #[clap(subcommand)]
    pub scrape_mode: ScrapeMode,

    /// Save a binary file
    pub binary: SaveBinary,
    
    /// Save a json file
    pub json: SaveJson,

    // Save to database
    pub database: SaveDatabase,
}

// continue by default
#[derive(Debug, Subcommand)]
pub enum ScrapeMode {
    ///
    Initialise,

    ///
    Continue,
}
