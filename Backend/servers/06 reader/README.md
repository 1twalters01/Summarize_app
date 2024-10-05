Reader microservice created purely in Rust.

# Why Rust
* A fast, smooth experience here is critical as it the main section of the app
* Will have a lot of throughput so low memory usage is desired
* Requires systems level things such as retrieving binary data
* No proprietary apis so there are no worries about rust support

# Overview
* Uses the polyglot app to convert files from structural data
* Converts to various file types for downloading to allow offline reading
* Store text colours, font, size, page colours, etc.
* Voiced reading if summary has it
* Optionally sync where user got to
    * Can opt out on a per book basis

# Details
* Store in a cache when document is being edited?
* saves the text and binary data as a (zipped?) folder
  * Saves it to S3?
  * rkyv crate to save binary
* Each chapter/section is in its own folder
* Each paragraph is in its own folder
* Web sockets allow users to collaborate in real time
* Lock each paragraph folder as it is being updated
