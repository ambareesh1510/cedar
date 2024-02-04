use std::{fs::read_to_string, collections::HashMap};

use crate::tokenizer::{generate_tokens, Token};

#[derive(Debug, Clone)]
pub enum AstNode {
    Tag {
        name: String,
        attributes: Vec<(String, String)>,
        children: Vec<AstNode>,
        custom: bool,
    },
    StringLiteral(String),
    Box(Vec<AstNode>),
    Children,
}

#[derive(Debug, PartialEq)]
pub enum ParseTokensError {
    Eof,
    SyntaxError,
    UndefinedSymbol,
}

fn get_next_attribute_token(tokens: &mut impl Iterator<Item = Token>) -> Result<Token, ParseTokensError> {
    let token = tokens.next();
    match token {
        None => Err(ParseTokensError::Eof),
        Some(t) => match t {
            Token::RightBracket => Err(ParseTokensError::SyntaxError),
            _ => Ok(t)
        }
    }
}


fn parse_attributes(tokens: &mut impl Iterator<Item = Token>) -> Result<Vec<(String, String)>, ParseTokensError> {
    let mut attributes = Vec::<(String, String)>::new();
    loop {
        let first_token = get_next_attribute_token(tokens);
        if let Err(_) = first_token {
            return Ok(attributes);
        }
        attributes.push(match (first_token.unwrap(), get_next_attribute_token(tokens)?, get_next_attribute_token(tokens)?) {
            // (a, b, c) if a == Some(Token::RightBracket) || b == Some(Token::RightBracket) || c == Some(Token::RightBracket) => return Ok(attributes),
            (Token::Keyword(k), Token::Equals, Token::StringLiteral(v)) => (k, v),
            _ => return Err(ParseTokensError::SyntaxError),
        })
    }
}

pub fn preprocess_tokens(tokens: &mut Vec<Token>) -> Result<(), ParseTokensError> {
    let mut index = 0;
    while index < tokens.len() {
        if let Token::Include = tokens[index] {
            if index == tokens.len() - 1 {
                return Err(ParseTokensError::Eof);
            }
            if let Token::StringLiteral(include_file_path) = &tokens[index + 1] {
                let Ok(include_file_source) = read_to_string(include_file_path) else {
                    println!("Include file not found: `{include_file_path}`");
                    return Err(ParseTokensError::SyntaxError);
                };
                let mut include_tokens = generate_tokens(&include_file_source);
                preprocess_tokens(&mut include_tokens)?;
                include_tokens.reverse();
                tokens.remove(index);
                tokens.remove(index);
                for token in include_tokens {
                    tokens.insert(index, token);
                }
            }
        }
        index += 1;
    }
    Ok(())
}

fn build_ast_node_from_context(tokens: &mut impl Iterator<Item = Token>, symbol_table: &mut HashMap<String, AstNode>) ->  Result<(Vec<(String, String)>, Vec<AstNode>), ParseTokensError> {
    let mut attributes = vec![];
    let children;
    match tokens.next() {
        None => return Err(ParseTokensError::Eof),
        Some(Token::LeftBracket) => {
            attributes = parse_attributes(tokens)?;
            if tokens.next() != Some(Token::LeftBrace) {
                return Err(ParseTokensError::SyntaxError);
            }
            children = parse_ast_node(tokens, symbol_table)?;
        },
        Some(Token::LeftBrace) => {
            children = parse_ast_node(tokens, symbol_table)?;
        },
        _ => return Err(ParseTokensError::SyntaxError),
    }
    Ok((attributes, children))
}

fn unwrap_component(name: &str, symbol_table: &HashMap<String, AstNode>, component_children: Vec<AstNode>) -> Result<AstNode, ParseTokensError> {
    let Some(AstNode::Tag {
        children,
        // attributes,
        ..
    }) = symbol_table.get(name) else {
        return Err(ParseTokensError::UndefinedSymbol);
    };
    // let component_children = component_children.into_iter();
    let mut unwrapped_children = vec![];
    for child in children {
        match child {
            AstNode::Tag {
                name,
                custom,
                children,
                ..
            } if *custom => {
                unwrapped_children.push(unwrap_component(name, symbol_table, children.clone())?);
            }
            AstNode::Children => {
                for component_child in component_children.clone() {
                    unwrapped_children.push(component_child);
                }
            }
            _ => unwrapped_children.push(child.clone()),
        }
    }
    Ok(AstNode::Box(unwrapped_children))
}

pub fn parse_ast_node(tokens: &mut impl Iterator<Item = Token>, symbol_table: &mut HashMap<String, AstNode>) -> Result<Vec<AstNode>, ParseTokensError> {
    let mut tags = Vec::<AstNode>::new();
    loop {
        tags.push(match tokens.next() {
            None => return Ok(tags),
            Some(token) => match token {
                Token::Children => AstNode::Children,
                Token::StringLiteral(s) => AstNode::StringLiteral(s),
                Token::Keyword(k) => {
                    let (attributes, children) = build_ast_node_from_context(tokens, symbol_table)?;
                    AstNode::Tag {
                        name: k,
                        attributes,
                        children,
                        custom: false,
                    }
                }
                Token::Def => {
                    let Some(Token::Keyword(k)) = tokens.next() else {
                        return Err(ParseTokensError::SyntaxError);
                    };
                    let (attributes, children) = build_ast_node_from_context(tokens, symbol_table)?;
                    symbol_table.insert(k.clone(), AstNode::Tag {
                        name: k,
                        attributes,
                        children,
                        custom: true,
                    });
                    continue;
                }
                Token::CustomComponent(k) => {
                    let (attributes, children) = build_ast_node_from_context(tokens, symbol_table)?;
                    unwrap_component(&k, symbol_table, children)?
                }
                Token::RightBrace => return Ok(tags),
                _ => todo!()
            }
        })
    }
}
