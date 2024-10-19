Microservice that managues all artist/book/genre data created in Python, Rust and C.

Why Python
###########
* Is going to be used less and less over time so load is going to be low
* Api speed is not a large issue
* Ease to maintain is important as metadata structure may change
    * Being a dynamic language helps massively in this case

Why Rust
#########
* Solid type system makes handling errors great
* Great async capabilities
* Easy to embed into python via maturin

Why C
######
* Is used for doing things that would be slow in python
* Is extremely easy to embed into python
* Very high speed and low memory usage

Overview
#########
* Manages the catalogue of books/authors/genres available
* Handles the metadata for the books/authors/genres
* Handles any changes to metadata structure and any database migrations
* Output all or part of the data in json or binary format
* Stores the number of summaries linked to each book as well as their location

Details
########
* Admins can automatically and manually add, update, or delete book/author/genre/publisher/etc. information
* Regular users can request to add or modify a book/author/genre/publisher
* Update the database in the case of any schema changes
* Uses the web_scraper app to get the information
