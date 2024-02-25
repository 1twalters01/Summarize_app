use polyglot::translate::txt::txt_file_to_document;
use std::{
    fs::File,
    io::Write,
};

pub fn test_file_to_document() {
    let document = txt_file_to_document("documents/text.txt").unwrap();
    println!("{:#?}", document);
    let text = document.to_txt();
    println!("{:#?}", text);

    let mut file = File::create("documents/text_out.txt").unwrap();
    file.write_all(text.unwrap().as_bytes()).unwrap();
}
