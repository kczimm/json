use std::{collections::HashMap, iter::Peekable};

use crate::{
    error::JsonError,
    tokenizer::{tokenize, Token},
    value::Value,
    Result,
};

pub fn parse(input: &str) -> Result<Value> {
    let mut tokens = tokenize(input)?.into_iter().peekable();
    parse_value(&mut tokens)
}

fn parse_value<I>(tokens: &mut Peekable<I>) -> Result<Value>
where
    I: Iterator<Item = Token>,
{
    let token = tokens.peek().ok_or(JsonError::NoTokens)?;

    match token {
        Token::LeftBrace => parse_object(tokens),
        Token::LeftBracket => parse_array(tokens),
        _ => {
            let token = tokens.next().unwrap(); // UNWRAP: we would have left after peek
            Ok(match token {
                Token::String(s) => Value::String(s),
                Token::Number(n) => Value::Number(n),
                Token::True => Value::Boolean(true),
                Token::False => Value::Boolean(false),
                Token::Null => Value::Null,
                t => return Err(JsonError::UnexpectedToken(Some(t.clone()))),
            })
        }
    }
}

fn parse_array<I>(tokens: &mut Peekable<I>) -> Result<Value>
where
    I: Iterator<Item = Token>,
{
    let token = tokens.next();
    if token != Some(Token::LeftBracket) {
        return Err(JsonError::UnexpectedToken(token));
    }

    let mut array = Vec::new();

    while let Some(token) = tokens.peek() {
        match token {
            Token::RightBracket => {
                tokens.next();
                break;
            }
            Token::Comma => {
                tokens.next();
                continue;
            }
            t => match t {
                Token::LeftBrace | Token::RightBrace | Token::LeftBracket | Token::Colon => {
                    return Err(JsonError::UnexpectedToken(Some(t.clone())))
                }
                _ => array.push(parse_value(tokens)?),
            },
        }
    }

    Ok(Value::Array(array))
}

fn parse_object<I>(tokens: &mut Peekable<I>) -> Result<Value>
where
    I: Iterator<Item = Token>,
{
    let token = tokens.next();
    if token != Some(Token::LeftBrace) {
        return Err(JsonError::UnexpectedToken(token));
    }

    let mut object = HashMap::new();

    while let Some(token) = tokens.next() {
        let key = match token {
            Token::RightBrace => break,
            Token::Comma => continue,
            Token::String(s) => s,
            t => return Err(JsonError::UnexpectedToken(Some(t))),
        };

        let token = tokens.next().unwrap();

        if token != Token::Colon {
            return Err(JsonError::UnexpectedToken(Some(token)));
        }

        let value = parse_value(tokens)?;

        object.insert(key, value);
    }

    Ok(Value::Object(object))
}

#[cfg(test)]
mod tests {
    use crate::value::COMPLETE_JSON;

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
                JsonError::UnexpectedToken(Some(token))
            );
        }
    }

    #[test]
    fn test_parse_value() {
        assert_eq!(
            parse_value(&mut [Token::True].into_iter().peekable()).unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            parse_value(&mut [Token::False].into_iter().peekable()).unwrap(),
            Value::Boolean(false)
        );
        assert_eq!(
            parse_value(&mut [Token::Null].into_iter().peekable()).unwrap(),
            Value::Null
        );
        assert_eq!(
            parse_value(&mut [Token::String("hello".to_string())].into_iter().peekable()).unwrap(),
            Value::String("hello".to_string())
        );
        assert_eq!(
            parse_value(&mut [Token::Number(-1.0)].into_iter().peekable()).unwrap(),
            Value::Number(-1.0)
        );
        assert_eq!(
            parse_value(&mut [Token::Number(1.1e3)].into_iter().peekable()).unwrap(),
            Value::Number(1100.0)
        );
    }

    #[test]
    fn test_parse_array() {
        assert_eq!(
            parse_array(
                &mut [
                    Token::LeftBracket,
                    Token::True,
                    Token::Comma,
                    Token::Null,
                    Token::RightBracket
                ]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            Value::Array(vec![Value::Boolean(true), Value::Null])
        );
        assert_eq!(
            parse_array(
                &mut [Token::LeftBracket, Token::RightBracket]
                    .into_iter()
                    .peekable()
            )
            .unwrap(),
            Value::Array(vec![])
        );
    }

    #[test]
    fn test_parse_object() {
        assert_eq!(
            parse_object(
                &mut [
                    Token::LeftBrace,
                    Token::String("key".to_string()),
                    Token::Colon,
                    Token::True,
                    Token::Comma,
                    Token::String("null".to_string()),
                    Token::Colon,
                    Token::Null,
                    Token::RightBrace
                ]
                .into_iter()
                .peekable()
            )
            .unwrap(),
            Value::Object(
                [
                    ("key".to_string(), Value::Boolean(true)),
                    ("null".to_string(), Value::Null)
                ]
                .into()
            )
        );

        assert_eq!(
            parse_object(&mut [Token::LeftBrace, Token::RightBrace].into_iter().peekable())
                .unwrap(),
            Value::Object([].into())
        );
    }

    #[test]
    fn test_parse_complete() {
        assert_eq!(
            parse(COMPLETE_JSON).unwrap(),
            Value::Object(
                [
                    (
                        "string".to_string(),
                        Value::String("This is a string".to_string())
                    ),
                    ("number".to_string(), Value::Number(42.0)),
                    ("float".to_string(), Value::Number(3.14)),
                    ("exponential".to_string(), Value::Number(2.998e8)),
                    ("negative".to_string(), Value::Number(-17.0)),
                    ("true".to_string(), Value::Boolean(true)),
                    ("false".to_string(), Value::Boolean(false)),
                    ("null".to_string(), Value::Null),
                    (
                        "array".to_string(),
                        Value::Array(vec![
                            Value::Number(1.0),
                            Value::Number(2.0),
                            Value::String("three".to_string()),
                            Value::Number(4.5),
                            Value::Boolean(true),
                            Value::Null,
                        ])
                    ),
                    (
                        "object".to_string(),
                        Value::Object(
                            [
                                ("key1".to_string(), Value::String("value1".to_string())),
                                ("key2".to_string(), Value::Number(100.0)),
                                (
                                    "key3".to_string(),
                                    Value::Object(
                                        [(
                                            "nested".to_string(),
                                            Value::String("object".to_string())
                                        )]
                                        .into()
                                    )
                                )
                            ]
                            .into()
                        )
                    ),
                    ("emptyArray".to_string(), Value::Array(Default::default())),
                    ("emptyObject".to_string(), Value::Object(Default::default()))
                ]
                .into()
            )
        );
    }
}
