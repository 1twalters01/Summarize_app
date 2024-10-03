use crate::datatypes::{Element, Node};
use std::fs::File;


// fix this
pub fn get_txt_metadata_from_file_name(file_path: &str) -> Option<Vec<Element>> {
    let file: File = File::open(file_path).unwrap();
    let mut metadata: Vec<Element> = Vec::new();

    let content: String = std::io::read_to_string(file).unwrap()
        .split('\n').collect::<String>()
        .split('\r').collect::<String>()
        .split('\t').collect::<String>();

    let characters = content.chars();
    let mut count = 0;

    for (index, character) in characters.enumerate() {

        if character == '\n' {
           let node: Node = Node {
               start: (index - count),
               end: (index - count),
               length: 1,
               id: None,
               class: None,
               parent: None,
               children: None,
               siblings_previous: None,
               siblings_next: None,
           };

           count += 1;
           metadata.push(Element::Newline(node));
        } else if character == '\r' {
           let node: Node = Node { 
               start: (index - count),
               end: (index - count),
               length: 1,
               id: None,
               class: None,
               parent: None,
               children: None,
               siblings_previous: None,
               siblings_next: None,
           };

           count += 1;
           metadata.push(Element::CarriageReturn(node));
        } else if character == '\t' {
           let node: Node = Node { 
               start: (index - count),
               end: (index - count),
               length: 1,
               id: None,
               class: None,
               parent: None,
               children: None,
               siblings_previous: None,
               siblings_next: None,
           };

           count += 1;
           metadata.push(Element::Tab(node));
        }
    }

    if metadata.is_empty() {
        return None;
    } else {
        return Some(metadata);
    }
}
