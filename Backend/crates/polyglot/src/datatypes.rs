use std::{
    cell::RefCell,
    rc::Weak,
    io::{Error, ErrorKind},
    result::Result,
};

#[derive(Debug)]
pub struct Document {
    pub name: String,
    pub file_type: DocType,
    pub content: String,
    pub metadata: Option<Vec<Element>>,
}

impl Document {
    pub fn new() -> Document {
        Document {
            name: String::new(),
            file_type: DocType::None,
            content: String::new(),
            metadata: None,
        }
    }

    pub fn to_txt(&self) -> Result<String, Error> {
        let mut content: String = self.content.clone();
        let mut count = 0;

        for (index, element) in self.metadata.as_ref().unwrap().iter().enumerate() {
            match &element {
                &Element::Newline(node) => {
                    let (first, second) = content.split_at(node.start + count);
                    content = first.to_owned() + "\n" + second;
                    count += 1;
                },
                &Element::CarriageReturn(node) => {
                    let (first, second) = content.split_at(node.start + count);
                    content = first.to_owned() + "\r" + second;
                    count += 1;
                },
                &Element::Tab(node) => {
                    let (first, second) = content.split_at(node.start + count);
                    content = first.to_owned() + "\t" + second;
                    count += 1;
                },
                _ => {}
            }
        }

        return Ok(content);
    }

    pub fn to_md(&self) -> Result<String, Error> {
        let mut content: String = self.content.clone();
        let count = 0;

        content = traverse_md_metadata(self.metadata.as_ref(), content, count).unwrap();

        return Ok(content);
    }


    pub fn to_rst(&self) -> Result<String, Error> {
        let mut content: String = self.content.clone();
        let count = 0;

        content = traverse_rst_metadata(self.metadata.as_ref(), content, count).unwrap();

        return Ok(content);
    }

    pub fn to_html(&self) -> Result<String, Error> {
        let mut content: String = self.content.clone();
        let count = 0;

        content = traverse_html_metadata(self.metadata.as_ref(), content, count).unwrap();

        return Ok(content);
    }

    pub fn to_xml(&self) -> Result<String, Error> {
        let mut content: String = self.content.clone();
        let count = 0;

        content = traverse_xml_metadata(self.metadata.as_ref(), content, count).unwrap();

        return Ok(content);
    }

    pub fn to_epub(&self) -> Result<String, Error> {
        let mut content: String = self.content.clone();
        let count = 0;

        content = traverse_epub_metadata(self.metadata.as_ref(), content, count).unwrap();


        return Ok(content);
    }

    pub fn to_pdf(&self) -> Result<String, Error> {
        let mut content: String = self.content.clone();
        let count = 0;

        content = traverse_pdf_metadata(self.metadata.as_ref(), content, count).unwrap();


        return Ok(content);
    }

    pub fn to_rtf(&self) -> Result<String, Error> {
        let mut content: String = self.content.clone();
        let count = 0;

        content = traverse_rtf_metadata(self.metadata.as_ref(), content, count).unwrap();

        return Ok(content);
    }

    pub fn to_latex(&self) -> Result<String, Error> {
        let mut content: String = self.content.clone();
        let count = 0;

        content = traverse_latex_metadata(self.metadata.as_ref(), content, count).unwrap();

        return Ok(content);
    }

    pub fn to_odf(&self) -> Result<String, Error> {
        let mut content: String = self.content.clone();
        let count = 0;

        content = traverse_odf_metadata(self.metadata.as_ref(), content, count).unwrap();

        return Ok(content);
    }

    pub fn to_docx(&self) -> Result<String, Error> {
        let mut content: String = self.content.clone();
        let count = 0;

        content = traverse_docx_metadata(self.metadata.as_ref(), content, count).unwrap();

        return Ok(content);
    }
}

fn traverse_md_metadata(metadata: Option<&Vec<Element>>, mut content: String, mut count: usize) -> Result<String, Error> {
    for (index, element) in metadata.unwrap().iter().enumerate() {
        match &element {
            &Element::Newline(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\n" + second;
                count += 1;
            },
            &Element::CarriageReturn(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\r" + second;
                count += 1;
            },
            &Element::Tab(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\t" + second;
                count += 1;

            },
            &Element::H1(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "# " + second;
                count += 2;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();

                match &node.id {
                    Some(id) => {
                        content = content + " {#" + id.as_str() + "}";
                        count += id.len() + 4;
                    },
                    None => {}
                }
            },
            &Element::H2(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "## " + second;
                count += 3;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();

                match &node.id {
                    Some(id) => {
                        content = content + " {#" + id.as_str() + "}";
                        count += id.len() + 4;
                    },
                    None => {}
                }
            },
            &Element::H3(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "### " + second;
                count += 4;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();

                match &node.id {
                    Some(id) => {
                        content = content + " {#" + id.as_str() + "}";
                        count += id.len() + 4;
                    },
                    None => {}
                }
            },
            &Element::H4(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "#### " + second;
                count += 5;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();

                match &node.id {
                    Some(id) => {
                        content = content + " {#" + id.as_str() + "}";
                        count += id.len() + 4;
                    },
                    None => {}
                }
            },
            &Element::H5(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "##### " + second;
                count += 6;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();

                match &node.id {
                    Some(id) => {
                        content = content + " {#" + id.as_str() + "}";
                        count += id.len() + 4;
                    },
                    None => {}
                }
            },
            &Element::H6(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "###### " + second;
                count += 7;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();

                match &node.id {
                    Some(id) => {
                        content = content + " {#" + id.as_str() + "}";
                        count += id.len() + 4;
                    },
                    None => {}
                }
            },
            &Element::Bold(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "**" + second;
                count += 2;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();

                content = content + "**";
                count += 2;
            },
            &Element::Italic(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "*" + second;
                count += 1;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            }
            &Element::Strikethrough(node) => {

            },
            &Element::Blockquote(node) => {

            },
            &Element::OrderedList(node) => {

            },
            &Element::UnorderedList(node) => {

            },
            // DefinitionList(node) => {

            //},
            &Element::Code(node) => {

            },
            &Element::HorizontalRule(node) => {

            },
            &Element::Link(node) => {

            },
            &Element::Image(node) => {

            },
            &Element::Table(node) => {

            },
            &Element::FencedCode(node) => {

            },
            &Element::Footnote(node) => {

            },
            &Element::DefintionList(node) => {

            },
            &Element::TaskList(node) => {

            },
            &Element::Emoji(node) => {

            },
            &Element::Highlight(node) => {

            },
            &Element::Subscript(node) => {

            },
            &Element::Superscript(node) => {

            },

            _ => {}
        }

    }

    return Ok(content);

}

pub fn traverse_rst_metadata(metadata: Option<&Vec<Element>>, mut content: String, mut count: usize) -> Result<String, Error> {
    for (index, element) in metadata.unwrap().iter().enumerate() {
        match &element {
            &Element::Newline(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\n" + second;
                count += 1;
            },
            &Element::CarriageReturn(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\r" + second;
                count += 1;
            },
            &Element::Tab(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\t" + second;
                count += 1;

            },
            &Element::H1(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "# " + second;
                count += 2;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H2(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "## " + second;
                count += 3;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H3(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "### " + second;
                count += 4;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H4(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "#### " + second;
                count += 5;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H5(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "##### " + second;
                count += 6;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H6(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "###### " + second;
                count += 7;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::Bold(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "**" + second;
                count += 2;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();

                content = content + "**";
                count += 2;
            },
            &Element::Italic(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "*" + second;
                count += 1;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            }

            _ => {}
        }

    }

    return Ok(content);
}

pub fn traverse_html_metadata(metadata: Option<&Vec<Element>>, mut content: String, mut count: usize) -> Result<String, Error> {
    for (index, element) in metadata.unwrap().iter().enumerate() {
        match &element {
            &Element::Newline(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\n" + second;
                count += 1;
            },
            &Element::CarriageReturn(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\r" + second;
                count += 1;
            },
            &Element::Tab(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\t" + second;
                count += 1;

            },
            &Element::H1(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "# " + second;
                count += 2;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H2(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "## " + second;
                count += 3;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H3(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "### " + second;
                count += 4;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H4(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "#### " + second;
                count += 5;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H5(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "##### " + second;
                count += 6;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H6(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "###### " + second;
                count += 7;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::Bold(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "**" + second;
                count += 2;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();

                content = content + "**";
                count += 2;
            },
            &Element::Italic(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "*" + second;
                count += 1;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            }

            _ => {}
        }

    }

    return Ok(content);
}

pub fn traverse_xml_metadata(metadata: Option<&Vec<Element>>, mut content: String, mut count: usize) -> Result<String, Error> {
    for (index, element) in metadata.unwrap().iter().enumerate() {
        match &element {
            &Element::Newline(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\n" + second;
                count += 1;
            },
            &Element::CarriageReturn(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\r" + second;
                count += 1;
            },
            &Element::Tab(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\t" + second;
                count += 1;

            },
            &Element::H1(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "# " + second;
                count += 2;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H2(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "## " + second;
                count += 3;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H3(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "### " + second;
                count += 4;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H4(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "#### " + second;
                count += 5;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H5(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "##### " + second;
                count += 6;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H6(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "###### " + second;
                count += 7;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::Bold(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "**" + second;
                count += 2;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();

                content = content + "**";
                count += 2;
            },
            &Element::Italic(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "*" + second;
                count += 1;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            }

            _ => {}
        }

    }

    return Ok(content);
}

pub fn traverse_epub_metadata(metadata: Option<&Vec<Element>>, mut content: String, mut count: usize) -> Result<String, Error> {
    for (index, element) in metadata.unwrap().iter().enumerate() {
        match &element {
            &Element::Newline(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\n" + second;
                count += 1;
            },
            &Element::CarriageReturn(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\r" + second;
                count += 1;
            },
            &Element::Tab(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\t" + second;
                count += 1;

            },
            &Element::H1(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "# " + second;
                count += 2;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H2(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "## " + second;
                count += 3;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H3(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "### " + second;
                count += 4;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H4(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "#### " + second;
                count += 5;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H5(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "##### " + second;
                count += 6;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H6(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "###### " + second;
                count += 7;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::Bold(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "**" + second;
                count += 2;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();

                content = content + "**";
                count += 2;
            },
            &Element::Italic(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "*" + second;
                count += 1;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            }

            _ => {}
        }

    }

    return Ok(content);
}

pub fn traverse_pdf_metadata(metadata: Option<&Vec<Element>>, mut content: String, mut count: usize) -> Result<String, Error> {
    for (index, element) in metadata.unwrap().iter().enumerate() {
        match &element {
            &Element::Newline(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\n" + second;
                count += 1;
            },
            &Element::CarriageReturn(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\r" + second;
                count += 1;
            },
            &Element::Tab(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\t" + second;
                count += 1;

            },
            &Element::H1(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "# " + second;
                count += 2;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H2(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "## " + second;
                count += 3;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H3(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "### " + second;
                count += 4;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H4(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "#### " + second;
                count += 5;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H5(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "##### " + second;
                count += 6;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H6(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "###### " + second;
                count += 7;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::Bold(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "**" + second;
                count += 2;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();

                content = content + "**";
                count += 2;
            },
            &Element::Italic(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "*" + second;
                count += 1;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            }

            _ => {}
        }

    }

    return Ok(content);
}

pub fn traverse_rtf_metadata(metadata: Option<&Vec<Element>>, mut content: String, mut count: usize) -> Result<String, Error> {
    for (index, element) in metadata.unwrap().iter().enumerate() {
        match &element {
            &Element::Newline(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\n" + second;
                count += 1;
            },
            &Element::CarriageReturn(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\r" + second;
                count += 1;
            },
            &Element::Tab(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\t" + second;
                count += 1;

            },
            &Element::H1(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "# " + second;
                count += 2;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H2(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "## " + second;
                count += 3;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H3(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "### " + second;
                count += 4;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H4(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "#### " + second;
                count += 5;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H5(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "##### " + second;
                count += 6;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H6(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "###### " + second;
                count += 7;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::Bold(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "**" + second;
                count += 2;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();

                content = content + "**";
                count += 2;
            },
            &Element::Italic(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "*" + second;
                count += 1;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            }

            _ => {}
        }

    }

    return Ok(content);
}

pub fn traverse_latex_metadata(metadata: Option<&Vec<Element>>, mut content: String, mut count: usize) -> Result<String, Error> {
    for (index, element) in metadata.unwrap().iter().enumerate() {
        match &element {
            &Element::Newline(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\n" + second;
                count += 1;
            },
            &Element::CarriageReturn(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\r" + second;
                count += 1;
            },
            &Element::Tab(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\t" + second;
                count += 1;

            },
            &Element::H1(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "# " + second;
                count += 2;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H2(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "## " + second;
                count += 3;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H3(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "### " + second;
                count += 4;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H4(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "#### " + second;
                count += 5;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H5(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "##### " + second;
                count += 6;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H6(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "###### " + second;
                count += 7;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::Bold(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "**" + second;
                count += 2;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();

                content = content + "**";
                count += 2;
            },
            &Element::Italic(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "*" + second;
                count += 1;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            }

            _ => {}
        }

    }

    return Ok(content);
}

pub fn traverse_odf_metadata(metadata: Option<&Vec<Element>>, mut content: String, mut count: usize) -> Result<String, Error> {
    for (index, element) in metadata.unwrap().iter().enumerate() {
        match &element {
            &Element::Newline(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\n" + second;
                count += 1;
            },
            &Element::CarriageReturn(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\r" + second;
                count += 1;
            },
            &Element::Tab(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\t" + second;
                count += 1;

            },
            &Element::H1(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "# " + second;
                count += 2;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H2(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "## " + second;
                count += 3;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H3(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "### " + second;
                count += 4;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H4(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "#### " + second;
                count += 5;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H5(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "##### " + second;
                count += 6;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H6(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "###### " + second;
                count += 7;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::Bold(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "**" + second;
                count += 2;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();

                content = content + "**";
                count += 2;
            },
            &Element::Italic(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "*" + second;
                count += 1;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            }

            _ => {}
        }

    }

    return Ok(content);
}

pub fn traverse_docx_metadata(metadata: Option<&Vec<Element>>, mut content: String, mut count: usize) -> Result<String, Error> {
    for (index, element) in metadata.unwrap().iter().enumerate() {
        match &element {
            &Element::Newline(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\n" + second;
                count += 1;
            },
            &Element::CarriageReturn(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\r" + second;
                count += 1;
            },
            &Element::Tab(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "\t" + second;
                count += 1;

            },
            &Element::H1(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "# " + second;
                count += 2;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H2(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "## " + second;
                count += 3;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H3(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "### " + second;
                count += 4;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H4(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "#### " + second;
                count += 5;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H5(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "##### " + second;
                count += 6;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::H6(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "###### " + second;
                count += 7;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            },
            &Element::Bold(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "**" + second;
                count += 2;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();

                content = content + "**";
                count += 2;
            },
            &Element::Italic(node) => {
                let (first, second) = content.split_at(node.start + count);
                content = first.to_owned() + "*" + second;
                count += 1;

                content = traverse_md_metadata(node.children.as_ref(), content, count).unwrap();
            }

            _ => {}
        }

    }

    return Ok(content);
}



#[derive(Debug)]
pub enum DocType {
    None,
    Txt,
    Md,
    Rst,
    Html,
    Xml,
    Epub,
    Pdf,
    Rtf,
    Latex,
    Odf,
    Docx,
}

#[derive(Debug)]
pub enum Element {
    Tab(Node),
    Newline(Node),
    CarriageReturn(Node),

    H1(Node),
    H2(Node),
    H3(Node),
    H4(Node),
    H5(Node),
    H6(Node),

    Bold(Node),
    Italic(Node),
    // Underline(Node),
    Blockquote(Node),
    OrderedList(Node),
    UnorderedList(Node),
    // DefinitionList(Node),
    Code(Node),
    HorizontalRule(Node),
    Link(Node),
    Image(Node),
    Table(Node),
    FencedCode(Node),
    Footnote(Node),
    DefintionList(Node),
    Strikethrough(Node),
    TaskList(Node),
    Emoji(Node),
    Highlight(Node),
    Subscript(Node),
    Superscript(Node),
}

#[derive(Debug)]
pub struct Node {
    pub start: usize,
    pub end: usize,
    pub length: usize,
    pub id: Option<String>,
    pub class: Option<String>,
    pub parent: Option<Weak<RefCell<Element>>>,
    pub siblings_previous: Option<Vec<Element>>,
    pub siblings_next: Option<Vec<Element>>,
    pub children: Option<Vec<Element>>,
}

