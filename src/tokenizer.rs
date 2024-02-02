#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Equals,
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
        v.push(Token::Keyword(b.clone()));
        b.clear();
    }
}
