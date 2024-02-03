use crate::tokenizer::Token;

#[derive(Debug)]
pub enum AstNode {
    Tag {
        name: String,
        attributes: Vec<(String, String)>,
        children: Vec<AstNode>,
    },
    StringLiteral(String),
}

#[derive(Debug, PartialEq)]
pub enum ParseTokensError {
    EOF,
    SyntaxError,
}

fn get_next_attribute_token(tokens: &mut impl Iterator<Item = Token>) -> Result<Token, ParseTokensError> {
    let token = tokens.next();
    match token {
        None => Err(ParseTokensError::EOF),
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

pub fn parse_ast_node(tokens: &mut impl Iterator<Item = Token>) -> Result<Vec<AstNode>, ParseTokensError> {
    let mut tags = Vec::<AstNode>::new();
    loop {
        tags.push(match tokens.next() {
            // None => return Err(ParseTokensError::EOF),
            None => return Ok(tags),
            Some(token) => match token {
                Token::StringLiteral(s) => AstNode::StringLiteral(s),
                Token::Keyword(k) => {
                    let mut attributes = vec![];
                    let children;
                    match tokens.next() {
                        None => return Err(ParseTokensError::EOF),
                        Some(Token::LeftBracket) => {
                            attributes = parse_attributes(tokens)?;
                            if tokens.next() != Some(Token::LeftBrace) {
                                return Err(ParseTokensError::SyntaxError);
                            }
                            children = parse_ast_node(tokens)?;
                        },
                        Some(Token::LeftBrace) => {
                            children = parse_ast_node(tokens)?;
                        },
                        _ => return Err(ParseTokensError::SyntaxError),
                    }
                    AstNode::Tag {
                        name: k,
                        attributes,
                        children,
                    }
                }
                Token::RightBrace => return Ok(tags),
                _ => todo!()
            }
        })
    }
}
