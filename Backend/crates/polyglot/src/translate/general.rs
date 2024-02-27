use crate::{
    datatypes::{Document, DocType, Element, Node},
    utils::translate::{
        get_name_from_filename,
        get_extension_from_filename,
    },
    translate::{
        txt::get_txt_metadata_from_file_name,
        md::get_md_metadata_from_file_name,
        rst::get_rst_metadata_from_file_name,
        html::get_html_metadata_from_file_name,
        xml::get_xml_metadata_from_file_name,
        epub::get_epub_metadata_from_file_name,
        pdf::get_pdf_metadata_from_file_name,
    }
};
use std::{
    fs::File,
    ffi::OsStr,
    path::Path,
    result::Result,
    io::{Error, ErrorKind}
};

pub fn file_to_document(file_path: &str) -> Result<Document, Error> {

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

    match get_extension_from_filename(file_path) {
        Some(file_type_str) => {
            match file_type_str {
                "txt" => {
                    document.file_type = DocType::Txt;
                    document.metadata = get_txt_metadata_from_file_name(file_path);
                },
                "md" => {
                    document.file_type = DocType::Md;
                    document.metadata = get_md_metadata_from_file_name(file_path);
                },
                "rst" => {
                    document.file_type = DocType::Rst;
                    document.metadata = get_rst_metadata_from_file_name(file_path);
                },
                "html" => {
                    document.file_type = DocType::Html;
                    document.metadata = get_html_metadata_from_file_name(file_path);
                },
                "xml" => {
                    document.file_type = DocType::Xml;
                    document.metadata = get_xml_metadata_from_file_name(file_path);
                },
                "epub" => {
                    document.file_type = DocType::Epub;
                    document.metadata = get_epub_metadata_from_file_name(file_path);
                },
                "pdf" => {
                    document.file_type = DocType::Pdf;
                    document.metadata = get_pdf_metadata_from_file_name(file_path);
                },
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

    return Ok(document);
}

