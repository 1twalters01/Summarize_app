Editor microservice created purely in Rust.

# Why Rust
* A fast, smooth experience here is critical as it is a main section of the app
* Will have a lot of throughput so low memory usage is desired
* Requires systems level things such as retrieving binary data
* No proprietary apis so there are no worries about rust support

# Overview
* Uses the polyglot app to convert files to and from structural data
* Links summaries to books
* Ensures that guidelines are met upon submission (piracy, explicit tag, etc.)
* Allows muliple people to edit at the same time
* Optional voice upload

# Details
* Save the text and binary data as a (zipped?) folder in S3
  * rkyv crate to save binary
  * Structure
    * Each chapter/section is in its own folder
    * Each paragraph is in its own folder
    * So Vec<Paragraph> (Chapter<Paragraph>)
* Store in a cache when document is being edited?
* Get document from cache or database
* Git-like change log upon being published between changes
* Choose if others can choose versions and which ones
  * Limit number of saved versions
