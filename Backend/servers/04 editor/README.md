Editor microservice created in Rust.

# Overview
* uses my polyglot crate to convert files to and from structural data

# Details
* Store in a cache when document is being edited?
* saves the text and binary data as a (zipped?) folder
  * Saves it to S3?
  * rkyv crate to save binary
* Each chapter/section is in its own folder
* Each paragraph is in its own folder
* Web sockets allow users to collaborate in real time
* Lock each paragraph folder as it is being updated
