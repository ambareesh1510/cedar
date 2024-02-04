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

pub fn write_html(file: &str, output_dir: &str, mut html: String) -> io::Result<()> {
    html.push_str("\n");
    let new_path = Path::new(output_dir).join(file).with_extension("html");
    let path_parent = new_path.parent().unwrap();
    fs::create_dir_all(path_parent)?;
    let write_html_result = fs::write(new_path.clone(), html);
    if write_html_result.is_ok() {
        println!("HTML written to {}", new_path.to_str().unwrap_or("<FILENAME CANNOT BE DISPLAYED>"));
    }
    write_html_result
}
