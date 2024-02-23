use std::{
    cell::RefCell,
    rc::Weak,
};

struct Document {
    name: String,
    file_type: DocType,
    content: String,
    metadata: Option<Vec<Node>>,
}

enum DocType {
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

struct Node {
    start: u64,
    end: u64,
    length: u64,
    id: String,
    class: String,
    parent: Option<Weak<RefCell<Node>>>,
    siblings_previous: Option<Vec<Node>>,
    siblings_next: Option<Vec<Node>>,
    children: Vec<Node>,
}

enum Elements {
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


