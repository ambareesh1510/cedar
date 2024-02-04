use crate::parser::AstNode;
use std::{fs, io, path::Path};

pub fn generate_html_from_ast(root: &Vec<AstNode>) -> String {
    let mut html = String::new();
    for node in root {
        match node {
            AstNode::StringLiteral(ref s) => html.push_str(s),
            AstNode::Tag { ref name, ref attributes, ref children, .. } => {
                html.push_str("<");
                html.push_str(name);
                // html.push_str(" ");
                for (k, v) in attributes {
                    html.push_str(" ");
                    html.push_str(k);
                    html.push_str("=\"");
                    html.push_str(v);
                    html.push_str("\"")
                }
                html.push_str(">");
                html.push_str(&generate_html_from_ast(children));
                html.push_str("</");
                html.push_str(name);
                html.push_str(">");
            }
            AstNode::Box(n) => {
                html.push_str(&generate_html_from_ast(n));
            }
            AstNode::Children => unreachable!(),
        }
    }
    html
}

pub fn write_html(file: &Path, mut html: String) -> io::Result<()> {
    html.push_str("\n");
    fs::write(&file, html)
}
