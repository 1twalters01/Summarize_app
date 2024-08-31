use crate::datatypes::{Element, Node};
use std::fs::File;



pub fn get_md_metadata_from_file_name(file_path: &str) -> Option<Vec<Element>> {
    let file: File = File::open(file_path).unwrap();
    let metadata: Vec<Element> = Vec::new();

    let content: String = std::io::read_to_string(file).unwrap()
        .split('\n').collect::<String>()
        .split('\r').collect::<String>()
        .split('\t').collect::<String>()
        .split("*").collect::<String>()
        .split("~").collect::<String>()
        .split("==").collect::<String>()
        .split('`').collect::<String>()
        .split('^').collect::<String>()
        .split("...").collect::<String>()
        .split("# ").collect::<String>()
        .split("## ").collect::<String>()
        .split("### ").collect::<String>()
        .split("#### ").collect::<String>()
        .split("##### ").collect::<String>()
        .split("###### ").collect::<String>();




    if metadata.is_empty() {
        return None;
    } else {
        return Some(metadata);
    }
}

