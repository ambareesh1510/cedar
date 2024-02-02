mod tokenizer;
mod parser;
mod generator;

use std::{env, fs::read_to_string};
use parser::ParseTokensError;

use crate::generator::{generate_html_from_ast, write_html};
use crate::tokenizer::{push_token, Token};
use crate::parser::parse_ast_node;

fn main() -> Result<(), ParseTokensError> {
    let source_path = env::args().nth(1).expect("No source file provided");
    println!("{source_path}");
    let source = read_to_string(source_path).expect(&format!("Could not read file"));

    let mut tokens: Vec<Token> = vec![];
    let mut string_buffer = String::from("");
    let mut string_parse_mode = false;
    for c in source.chars() {
        if !string_parse_mode {
            match c {
                c if c.is_whitespace() => {
                    push_token(&mut tokens, &mut string_buffer);
                }
                '[' | ']' | '{' | '}' | '=' => {
                    push_token(&mut tokens, &mut string_buffer);
                    tokens.push(c.into());
                }
                '"' => {
                    push_token(&mut tokens, &mut string_buffer);
                    string_parse_mode = true;
                }
                _ => string_buffer.push(c),
            }
        } else {
            if c == '"' {
                tokens.push(Token::StringLiteral(string_buffer.clone()));
                string_buffer.clear();
                string_parse_mode = false;
            } else {
                string_buffer.push(c);
            }
        }
    }
    let ast = parse_ast_node(&mut tokens.into_iter())?;
    let html = generate_html_from_ast(&ast);
    write_html("test/index.html", html).unwrap();
    Ok(())
}
