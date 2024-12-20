use std::{fmt, iter::Peekable, str::Chars};

use crate::{error::JsonError, Result};

#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    String(String),
    Number(f64),
    True,
    False,
    Null,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut chars = Indexer::new(input.chars().peekable());
    while let Some(c) = chars.next() {
        match c {
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            '[' => tokens.push(Token::LeftBracket),
            ']' => tokens.push(Token::RightBracket),
            '"' => {
                let mut s = String::new();
                while let Some(c) = chars.next() {
                    if c == '"' {
                        // end of string
                        break;
                    }
                    s.push(c);
                }
                tokens.push(Token::String(s));
            }
            ',' => tokens.push(Token::Comma),
            ':' => tokens.push(Token::Colon),
            't' => {
                // expecting literal true
                chars_match(&mut chars, Token::True)?;
                tokens.push(Token::True);
            }
            'f' => {
                // expecting literal false
                chars_match(&mut chars, Token::False)?;
                tokens.push(Token::False);
            }
            'n' => {
                // expecting literal null
                chars_match(&mut chars, Token::Null)?;
                tokens.push(Token::Null);
            }
            c if c.is_digit(10) || c == '-' => {
                static NUM_CHARS: &[char] = &['.', 'e', 'E', '+', '-'];
                let mut num = String::new();
                num.push(c);
                while let Some(&c) = chars.chars.peek() {
                    if c.is_digit(10) || NUM_CHARS.contains(&c) {
                        num.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let num = num.parse().unwrap();
                tokens.push(Token::Number(num));
            }
            ' ' | '\n' => {}
            c => {
                return Err(JsonError::UnexpectedCharacter {
                    position: chars.position(),
                    expected_token: None,
                    got: Some(c),
                })
            }
        }
    }

    Ok(tokens)
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::LeftBracket => write!(f, "["),
            Token::RightBracket => write!(f, "]"),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::String(s) => write!(f, "{s}"),
            Token::Number(n) => write!(f, "{n}"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Null => write!(f, "null"),
        }
    }
}

struct Indexer<'a> {
    chars: Peekable<Chars<'a>>,
    row: usize,
    column: usize,
}

impl<'a> Indexer<'a> {
    fn new(chars: Peekable<Chars<'a>>) -> Self {
        Self {
            chars,
            row: 0,
            column: 0,
        }
    }

    fn position(&self) -> (usize, usize) {
        (self.row, self.column)
    }
}

impl<'a> Iterator for Indexer<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.chars.next();
        match next {
            Some('\n') => {
                self.row += 1;
                self.column = 0;
            }
            Some(_) => {
                self.column += 1;
            }
            None => {}
        }
        next
    }
}

fn chars_match(chars: &mut Indexer, expected_token: Token) -> Result<Token> {
    for c in expected_token.to_string().chars().skip(1) {
        let got = chars.next();
        if got != Some(c) {
            return Err(JsonError::UnexpectedCharacter {
                position: (chars.row, chars.column),
                expected_token: Some(expected_token),
                got,
            });
        }
    }

    Ok(expected_token)
}

#[cfg(test)]
mod tests {
    use super::*;

    static COMPLETE_JSON: &str = r#"{
    "string": "This is a string",
    "number": 42,
    "float": 3.14,
    "exponential": 2.998e8,
    "negative": -17,
    "true": true,
    "false": false,
    "null": null,
    "array": [
        1,
        2,
        "three",
        4.5,
        true,
        null
    ],
    "object": {
        "key1": "value1",
        "key2": 100,
        "key3": {
            "nested": "object"
        }
    },
    "emptyArray": [],
    "emptyObject": {}
}"#;

    #[test]
    fn test_tokenize() {
        let tokens = tokenize(COMPLETE_JSON).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LeftBrace,
                Token::String("string".to_string()),
                Token::Colon,
                Token::String("This is a string".to_string()),
                Token::Comma,
                Token::String("number".to_string()),
                Token::Colon,
                Token::Number(42.0),
                Token::Comma,
                Token::String("float".to_string()),
                Token::Colon,
                Token::Number(3.14),
                Token::Comma,
                Token::String("exponential".to_string()),
                Token::Colon,
                Token::Number(299800000.0),
                Token::Comma,
                Token::String("negative".to_string()),
                Token::Colon,
                Token::Number(-17.0),
                Token::Comma,
                Token::String("true".to_string()),
                Token::Colon,
                Token::True,
                Token::Comma,
                Token::String("false".to_string()),
                Token::Colon,
                Token::False,
                Token::Comma,
                Token::String("null".to_string()),
                Token::Colon,
                Token::Null,
                Token::Comma,
                Token::String("array".to_string()),
                Token::Colon,
                Token::LeftBracket,
                Token::Number(1.0),
                Token::Comma,
                Token::Number(2.0),
                Token::Comma,
                Token::String("three".to_string()),
                Token::Comma,
                Token::Number(4.5),
                Token::Comma,
                Token::True,
                Token::Comma,
                Token::Null,
                Token::RightBracket,
                Token::Comma,
                Token::String("object".to_string()),
                Token::Colon,
                Token::LeftBrace,
                Token::String("key1".to_string()),
                Token::Colon,
                Token::String("value1".to_string()),
                Token::Comma,
                Token::String("key2".to_string()),
                Token::Colon,
                Token::Number(100.0),
                Token::Comma,
                Token::String("key3".to_string()),
                Token::Colon,
                Token::LeftBrace,
                Token::String("nested".to_string()),
                Token::Colon,
                Token::String("object".to_string()),
                Token::RightBrace,
                Token::RightBrace,
                Token::Comma,
                Token::String("emptyArray".to_string()),
                Token::Colon,
                Token::LeftBracket,
                Token::RightBracket,
                Token::Comma,
                Token::String("emptyObject".to_string()),
                Token::Colon,
                Token::LeftBrace,
                Token::RightBrace,
                Token::RightBrace
            ]
        );
    }

    #[test]
    fn test_unexpected_character() {
        let input = r#"{
"input": [trueeeeee, false]
}"#;
        assert_eq!(
            tokenize(input).unwrap_err(),
            JsonError::UnexpectedCharacter {
                position: (1, 15),
                expected_token: None,
                got: Some('e')
            }
        );

        let input = r#"{
"input": [true, fallse]
}"#;
        assert_eq!(
            tokenize(input).unwrap_err(),
            JsonError::UnexpectedCharacter {
                position: (1, 20),
                expected_token: Some(Token::False),
                got: Some('l')
            }
        );
    }
}
