use std::fmt;

use crate::tokenizer::Token;

type Row = usize;
type Column = usize;

#[derive(Debug, PartialEq)]
pub enum JsonError {
    ParsingNumber {
        position: (Row, Column),
        message: String,
    },
    UnexpectedCharacter {
        position: (Row, Column),
        expected_token: Option<Token>,
        got: Option<char>,
    },
    UnexpectedToken(Token),
    NoTokens,
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
