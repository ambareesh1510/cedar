mod tokenizer;
mod parser;
mod generator;
mod cli;

use std::{fs::read_to_string, path::Path, collections::HashMap};

use cli::parse_command_line_args;
use parser::{ParseTokensError, parse_ast_node};
use tokenizer::generate_tokens;
use generator::{generate_html_from_ast, write_html};

use crate::parser::preprocess_tokens;

fn main() -> Result<(), ParseTokensError> {
    let command_line_args = parse_command_line_args();

    for path in command_line_args.source_path {
        let source = read_to_string(path.clone()).expect("Could not read file");
        let mut tokens = generate_tokens(&source);
        preprocess_tokens(&mut tokens)?;
        println!("{:?}", tokens);

        let ast = parse_ast_node(&mut tokens.into_iter(), &mut HashMap::new())?;
        let html = generate_html_from_ast(&ast);
        let new_path = Path::new(&path).with_extension("html");
        write_html(&new_path, html).unwrap();
        println!("HTML written to {}", new_path.to_str().unwrap_or("<FILENAME CANNOT BE DISPLAYED>"));
    }
    Ok(())
}
