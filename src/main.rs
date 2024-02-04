mod tokenizer;
mod parser;
mod generator;
mod cli;

use std::{collections::HashMap, error::Error, fs::read_to_string, path::Path};

use cli::parse_command_line_args;
use parser::parse_ast_node;
use tokenizer::generate_tokens;
use generator::{generate_html_from_ast, write_html};

use crate::parser::preprocess_tokens;

fn main() -> Result<(), Box<dyn Error>> {
    let command_line_args = parse_command_line_args()?;

    for path in command_line_args.input_paths {
        let source = read_to_string(path.clone()).expect("Could not read file");
        let mut tokens = generate_tokens(&source);
        preprocess_tokens(&mut tokens, &command_line_args.include_dirs)?;

        let ast = parse_ast_node(&mut tokens.into_iter(), &mut HashMap::new())?;
        let html = generate_html_from_ast(&ast);
        // let new_path = Path::new(&path).with_extension("html");
        write_html(&path, &command_line_args.output_dir, html).unwrap();
    }
    Ok(())
}
