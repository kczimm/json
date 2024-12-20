use std::collections::HashMap;

use crate::{
    error::JsonError,
    tokenizer::{tokenize, Token},
    value::Value,
    Result,
};

pub fn parse(input: &str) -> Result<Value> {
    let mut tokens = tokenize(input)?.into_iter();
    parse_value(&mut tokens)
}

fn parse_value(tokens: &mut impl Iterator<Item = Token>) -> Result<Value> {
    let token = tokens.next().ok_or(JsonError::NoTokens)?;
    let value = match dbg!(token) {
        Token::LeftBrace => parse_object(tokens)?,
        Token::LeftBracket => parse_array(tokens)?,
        Token::String(s) => Value::String(s),
        Token::Number(n) => Value::Number(n),
        Token::True => Value::Boolean(true),
        Token::False => Value::Boolean(false),
        Token::Null => Value::Null,
        t => return Err(JsonError::UnexpectedToken(t)),
    };

    Ok(value)
}

fn parse_array(tokens: &mut impl Iterator<Item = Token>) -> Result<Value> {
    let mut array = Vec::new();

    while let Some(token) = tokens.next() {
        match token {
            Token::RightBracket => break,
            Token::Comma => continue,
            t => match t {
                Token::LeftBrace | Token::RightBrace | Token::LeftBracket | Token::Colon => {
                    return Err(JsonError::UnexpectedToken(t))
                }
                _ => array.push(parse_value(tokens)?),
            },
        }
    }

    Ok(Value::Array(array))
}

fn parse_object(tokens: &mut impl Iterator<Item = Token>) -> Result<Value> {
    let mut object = HashMap::new();

    while let Some(token) = tokens.next() {
        let key = match token {
            Token::RightBrace => break,
            Token::String(s) => s,
            t => return Err(JsonError::UnexpectedToken(t)),
        };

        let token = tokens.next().unwrap();

        if token != Token::Colon {
            return Err(JsonError::UnexpectedToken(token));
        }

        let value = parse_value(tokens)?;

        object.insert(key, value);
    }

    Ok(Value::Object(object))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_no_tokens() {
        assert_eq!(parse("").unwrap_err(), JsonError::NoTokens);
    }

    #[test]
    fn test_parse_unexpected_token() {
        for token in [
            Token::RightBrace,
            Token::RightBracket,
            Token::Comma,
            Token::Colon,
        ] {
            let input = token.to_string();
            assert_eq!(
                parse(&input).unwrap_err(),
                JsonError::UnexpectedToken(token)
            );
        }
    }

    #[test]
    fn test_parse_value() {
        assert_eq!(
            parse_value(&mut [Token::True].into_iter()).unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            parse_value(&mut [Token::False].into_iter()).unwrap(),
            Value::Boolean(false)
        );
        assert_eq!(
            parse_value(&mut [Token::Null].into_iter()).unwrap(),
            Value::Null
        );
        assert_eq!(
            parse_value(&mut [Token::String("hello".to_string())].into_iter()).unwrap(),
            Value::String("hello".to_string())
        );
        assert_eq!(
            parse_value(&mut [Token::Number(-1.0)].into_iter()).unwrap(),
            Value::Number(-1.0)
        );
        assert_eq!(
            parse_value(&mut [Token::Number(1.1e3)].into_iter()).unwrap(),
            Value::Number(1100.0)
        );
    }

    #[test]
    fn test_parse_array() {
        assert_eq!(
            parse_array(
                &mut [
                    // Token::LeftBracket, // this is already stripped when calling parse_array
                    Token::True,
                    Token::Comma,
                    Token::Null,
                    Token::RightBracket
                ]
                .into_iter()
            )
            .unwrap(),
            Value::Array(vec![Value::Boolean(true), Value::Null])
        );
    }
}
