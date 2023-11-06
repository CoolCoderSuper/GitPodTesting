use std::ops::Index;
//TODO: Name in property
//TODO: Actual value parsing
//TODO: Real AST
//TODO: Error recovery
use crate::{lexer::*, syntax::*, token::*};

const NO_TOKENS: &str = "No tokens left.";

pub struct Parser {
    pub tokens: Vec<SyntaxToken>,
    position: usize,
}

impl Parser {
    pub fn new(text: &str) -> Parser {
        let mut lexer = Lexer::new(text);
        let mut tokens: Vec<SyntaxToken> = Vec::new();
        loop {
            let token = lexer.lex();
            if token.kind == SyntaxKind::EndOfFileToken {
                break;
            } else if token.kind != SyntaxKind::BadToken && token.kind != SyntaxKind::WhiteSpaceToken {
                tokens.push(token);
            }
        }
        Parser {
            tokens,
            position: 0,
        }
    }

    fn next(&mut self) {
        self.skip(1)
    }

    fn skip(&mut self, offset: usize) {
        self.position += offset
    }

    fn current(&self) -> Option<&SyntaxToken> {
        self.peek(0)
    }

    fn peek(&self, offset: usize) -> Option<&SyntaxToken> {
        let index = self.position + offset;
        if index >= self.tokens.len() {
            return None;
        }
        Some(self.tokens.index(index))
    }

    pub fn parse(&mut self) -> Result<JsonValue, String> {
        match self.current() {
            Some(token) => match token.kind {
                SyntaxKind::TrueLiteralToken => self.parse_true(),
                SyntaxKind::FalseLiteralToken => self.parse_false(),
                SyntaxKind::NullLiteralToken => self.parse_null(),
                SyntaxKind::NumberLiteralToken => self.parse_number(),
                SyntaxKind::StringLiteralToken => self.parse_string(),
                SyntaxKind::OpenBracketToken => self.parse_array(),
                SyntaxKind::OpenBraceToken => self.parse_object(),
                _ => Err("Failed to match any tokens.".to_string()),
            },
            None => Err(NO_TOKENS.to_string()),
        }
    }

    fn parse_true(&mut self) -> Result<JsonValue, String> {
        match self.current() {
            Some(token) => {
                if token.kind == SyntaxKind::TrueLiteralToken {
                    let val = JsonValue::Boolean(true);
                    self.next();
                    return Ok(val);
                }
                Err(format!(
                    "Expected 'TrueLiteralToken' but got '{:?}'",
                    token.kind
                ))
            }
            None => Err(NO_TOKENS.to_string()),
        }
    }

    fn parse_false(&mut self) -> Result<JsonValue, String> {
        match self.current() {
            Some(token) => {
                if token.kind == SyntaxKind::FalseLiteralToken {
                    let val = JsonValue::Boolean(false);
                    self.next();
                    return Ok(val);
                }
                Err(format!(
                    "Expected 'FalseLiteralToken' but got '{:?}'",
                    token.kind
                ))
            }
            None => Err(NO_TOKENS.to_string()),
        }
    }

    fn parse_null(&mut self) -> Result<JsonValue, String> {
        match self.current() {
            Some(token) => {
                if token.kind == SyntaxKind::NullLiteralToken {
                    let val = JsonValue::Null;
                    self.next();
                    return Ok(val);
                }
                Err(format!(
                    "Expected 'NullLiteralToken' but got '{:?}'",
                    token.kind
                ))
            }
            None => Err(NO_TOKENS.to_string()),
        }
    }

    fn parse_number(&mut self) -> Result<JsonValue, String> {
        match self.current() {
            Some(token) => {
                if token.kind == SyntaxKind::NumberLiteralToken {
                    let val = JsonValue::Number(JsonNumber {
                        text: token.text.clone(),
                        value: 0.0,
                    });
                    self.next();
                    return Ok(val);
                }
                Err(format!(
                    "Expected 'NumberLiteralToken' but got '{:?}'",
                    token.kind
                ))
            }
            None => Err(NO_TOKENS.to_string()),
        }
    }

    fn parse_string(&mut self) -> Result<JsonValue, String> {
        match self.current() {
            Some(token) => {
                if token.kind == SyntaxKind::StringLiteralToken {
                    let val = JsonValue::String(JsonString {
                        text: token.text.clone(),
                        value: "".to_string(),
                    });
                    self.next();
                    return Ok(val);
                }
                Err(format!(
                    "Expected 'StringLiteralToken' but got '{:?}'",
                    token.kind
                ))
            }
            None => Err(NO_TOKENS.to_string()),
        }
    }

    fn parse_array(&mut self) -> Result<JsonValue, String> {
        match self.current() {
            Some(token) => {
                if token.kind == SyntaxKind::OpenBracketToken {
                    self.next();
                    let mut vals: Vec<JsonValue> = Vec::new();
                    while let Some(comma_token) = self.current() {
                        if comma_token.kind == SyntaxKind::CommaToken {
                            self.next();
                        } else if comma_token.kind == SyntaxKind::CloseBracketToken {
                            self.next();
                            break;
                        } else {
                            match self.parse() {
                                Ok(val) => {
                                    vals.push(val);
                                }
                                Err(e) => return Err(e),
                            }
                        }
                    }
                    return Ok(JsonValue::Array(JsonArray { elements: vals }));
                } else {
                    Err(format!(
                        "Expected 'OpenBracketToken' but got '{:?}'",
                        token.kind
                    ))
                }
            }
            None => Err(NO_TOKENS.to_string()),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
        match self.current() {
            Some(token) => {
                if token.kind == SyntaxKind::OpenBraceToken {
                    self.next();
                    let mut props: Vec<JsonProperty> = Vec::new();
                    while let Some(comma_token) = self.current() {
                        if comma_token.kind == SyntaxKind::CommaToken {
                            self.next();
                        } else if comma_token.kind == SyntaxKind::CloseBraceToken {
                            self.next();
                            break;
                        } else {
                            if comma_token.kind == SyntaxKind::StringLiteralToken || comma_token.kind == SyntaxKind::IdentifierToken {
                                self.next();
                                if self.current().is_some(){
                                    self.next();
                                }
                                match self.parse() {
                                    Ok(val) => {
                                        props.push(JsonProperty { name: "".to_string(), value: val })
                                    },
                                    Err(e) => return Err(e)
                                }
                            }
                        }
                    }
                    return Ok(JsonValue::Object(JsonObject { properties: props }));
                } else {
                    Err(format!(
                        "Expected 'OpenBraceToken' but got '{:?}'",
                        token.kind
                    ))
                }
            }
            None => Err(NO_TOKENS.to_string()),
        }
    }
}
