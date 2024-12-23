use crate::tokenizer::Token;

pub type Position = (Row, Column);

type Row = usize;
type Column = usize;

#[derive(Debug, PartialEq)]
pub enum JsonError {
    ParsingNumber {
        position: Position,
        message: String,
    },
    UnexpectedCharacter {
        position: Position,
        expected_token: Option<Token>,
        got: Option<char>,
    },
    UnexpectedToken(Option<Token>),
    NoTokens,
}
