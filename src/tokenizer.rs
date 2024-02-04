#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Equals,
    Include,
    Def,
    Children,
    CustomComponent(String),
    Keyword(String),
    StringLiteral(String),
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '[' => Self::LeftBracket,
            ']' => Self::RightBracket,
            '{' => Self::LeftBrace,
            '}' => Self::RightBrace,
            '=' => Self::Equals,
            _ => panic!("Failed conversion from char to Token: char {value} could not be converted")
        }
    }
}

pub fn push_token(v: &mut Vec<Token>, b: &mut String) {
    if b.len() != 0 {
        if b == "@include" {
            v.push(Token::Include);
        } else if b == "@def" {
            v.push(Token::Def);
        } else if b == "@children" {
            v.push(Token::Children);
        } else if &b[0..1] == "@" {
            v.push(Token::CustomComponent(b[1..].to_string()))
        } else {
            v.push(Token::Keyword(b.clone()));
        }
        b.clear();
    }
}

pub fn generate_tokens(source: &str) -> Vec<Token> {
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
    tokens
}
