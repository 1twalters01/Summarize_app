use crate::datatypes::{Document, DocType, Element, Node};
use std::{
    fs::File,
    ffi::OsStr,
    path::Path,
    result::Result,
    io::{Error, ErrorKind, Read}
};

pub fn txt_file_to_document(file_path: &str) -> Result<Document, Error> {
    let file: File = File::open(file_path).unwrap();

    let mut document: Document = Document::new();

    document.name = match get_name_from_filename(file_path) {
        Some(name) => name.to_string(),
        None => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Could not find a name for the file: {}", file_path)
            ));
        },
    };

    document.file_type = match get_extension_from_filename(file_path) {
        Some(file_type_str) => {
            match file_type_str {
                "txt" => DocType::Txt,
                "md" => DocType::Md,
                "rst" => DocType::Rst,
                "html" => DocType::Html,
                "xml" => DocType::Xml,
                "epub" => DocType::Epub,
                "pdf" => DocType::Pdf,
                "rtf" => DocType::Rtf,
                "latex" => DocType::Latex,
                "odf" => DocType::Odf,
                "docx" => DocType::Docx,
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("Could not find a file type for the file: {}", file_path)
                    ));
                }
            }
        },
        None => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Could not find a file type for the file: {}", file_path)
            ));
        }
    };

    let content: String = std::io::read_to_string(file).unwrap();
    document.content = content
        .split('\n').collect::<String>()
        .split('\r').collect::<String>()
        .split('\t').collect::<String>();

    document.metadata = get_txt_metadata_from_string(content);

    return Ok(document);
}

pub fn get_name_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
     .file_name()
     .and_then(OsStr::to_str)
}

pub fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
    .extension()
    .and_then(OsStr::to_str)
}

pub fn get_txt_metadata_from_string(content: String) -> Option<Vec<Element>> {
    let characters = content.chars();
    let mut metadata: Vec<Element> = Vec::new();
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
