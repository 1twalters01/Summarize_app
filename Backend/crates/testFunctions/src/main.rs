pub mod polyglot_test;
extern crate polyglot;
use polyglot_test::experiments::{
    test_txt_file_to_document,
    test_md_file_to_document,
    test_rst_file_to_document,
    test_html_file_to_document,
    test_xml_file_to_document,
    test_epub_file_to_document,
    test_pdf_file_to_document,
};

fn main() {
    test_txt_file_to_document();
    test_md_file_to_document();
    // test_rst_file_to_document();
    // test_html_file_to_document();
    // test_xml_file_to_document();
    // test_epub_file_to_document();
    // test_pdf_file_to_document();
}
