use polyglot::{
    datatypes::Document,
    translate::general::file_to_document,
};
use std::{
    fs::File,
    io::Write,
};

pub fn test_txt_file_to_document() {
    let document: Document = file_to_document("documents/text.txt").unwrap();
    println!("{:#?}", document);
    let text = document.to_txt();
    println!("{:#?}", text);

    let mut file = File::create("documents/text_out.txt").unwrap();
    file.write_all(text.unwrap().as_bytes()).unwrap();
}

pub fn test_md_file_to_document() {
    let document: Document = file_to_document("documents/text.md").unwrap();
    println!("{:#?}", document);
    let text = document.to_md();
    println!("{:#?}", text);

    let mut file = File::create("documents/text_out.md").unwrap();
    file.write_all(text.unwrap().as_bytes()).unwrap();
}

pub fn test_rst_file_to_document() {
    let document: Document = file_to_document("documents/text.rst").unwrap();
    println!("{:#?}", document);
    let text = document.to_rst();
    println!("{:#?}", text);

    let mut file = File::create("documents/text_out.rst").unwrap();
    file.write_all(text.unwrap().as_bytes()).unwrap();
}

pub fn test_html_file_to_document() {
    let document: Document = file_to_document("documents/text.html").unwrap();
    println!("{:#?}", document);
    let text = document.to_html();
    println!("{:#?}", text);

    let mut file = File::create("documents/text_out.html").unwrap();
    file.write_all(text.unwrap().as_bytes()).unwrap();
}

pub fn test_xml_file_to_document() {
    let document: Document = file_to_document("documents/text.xml").unwrap();
    println!("{:#?}", document);
    let text = document.to_xml();
    println!("{:#?}", text);

    let mut file = File::create("documents/text_out.xml").unwrap();
    file.write_all(text.unwrap().as_bytes()).unwrap();
}

pub fn test_epub_file_to_document() {
    let document: Document = file_to_document("documents/text.epub").unwrap();
    println!("{:#?}", document);
    let text = document.to_epub();
    println!("{:#?}", text);

    let mut file = File::create("documents/text_out.epub").unwrap();
    file.write_all(text.unwrap().as_bytes()).unwrap();
}

pub fn test_pdf_file_to_document() {
    let document: Document = file_to_document("documents/text.pdf").unwrap();
    println!("{:#?}", document);
    let text = document.to_pdf();
    println!("{:#?}", text);

    let mut file = File::create("documents/text_out.pdf").unwrap();
    file.write_all(text.unwrap().as_bytes()).unwrap();
}


