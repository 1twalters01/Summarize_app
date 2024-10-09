use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct ScraperArgs {
    /// Scrape mode
    #[clap(subcommand)]
    pub scrape_mode: ScrapeMode,

    /// Save a binary file
    pub binary: bool,
    
    /// Save a json file
    pub json: bool,

    // Save to database
    pub database: bool,
}

// continue by default
#[derive(Debug, Subcommand)]
pub enum ScrapeMode {
    ///
    Initialise,

    ///
    Continue,
}
