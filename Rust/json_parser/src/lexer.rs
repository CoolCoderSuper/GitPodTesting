use crate::token::*;
pub struct Lexer {
    text: String,
    position: usize,
}

impl Lexer {
    pub fn new(text: &str) -> Lexer {
        Lexer {
            text: text.to_string(),
            position: 0,
        }
    }

    fn current(&self) -> char {
        self.peek(0)
    }

    fn next(&mut self) {
        self.skip(1)
    }

    fn skip(&mut self, offest: usize) {
        self.position += offest
    }

    fn peek(&self, offest: usize) -> char {
        let index = self.position + offest;
        if index >= self.text.len() {
            return '\0';
        }
        self.text.chars().nth(index).unwrap()
    }

    pub fn lex(&mut self) -> SyntaxToken {
        if self.matches_eof() {
            return SyntaxToken {
                text: "".to_string(),
                position: self.position,
                kind: SyntaxKind::EndOfFileToken,
            };
        }
        if let Some(token) = self.match_whitespace() {
            return token;
        }
        if let Some(token) = self.match_openbrace() {
            return token;
        }
        if let Some(token) = self.match_closebrace() {
            return token;
        }
        if let Some(token) = self.match_openbracket() {
            return token;
        }
        if let Some(token) = self.match_closebracket() {
            return token;
        }
        if let Some(token) = self.match_comma() {
            return token;
        }
        if let Some(token) = self.match_colon() {
            return token;
        }
        if let Some(token) = self.match_true() {
            return token;
        }
        if let Some(token) = self.match_false() {
            return token;
        }
        if let Some(token) = self.match_null() {
            return token;
        }
        if let Some(token) = self.match_number() {
            return token;
        }
        if let Some(token) = self.match_string() {
            return token;
        }
        if let Some(token) = self.match_identifier() {
            return token;
        }
        if let Some(token) = self.match_comment() {
            return token;
        }
        if let Some(token) = self.match_blockcomment() {
            return token;
        }
        let bad = SyntaxToken {
            text: self.current().to_string(),
            position: self.position,
            kind: SyntaxKind::BadToken,
        };
        self.next();
        bad
    }

    fn matches_eof(&self) -> bool {
        self.current() == '\0'
    }

    fn match_whitespace(&mut self) -> Option<SyntaxToken> {
        if self.current().is_whitespace() {
            let start = self.position;
            while self.current().is_whitespace() {
                self.next()
            }
            return Some(SyntaxToken {
                text: self.text[start..self.position].to_string(),
                position: start,
                kind: SyntaxKind::WhiteSpaceToken,
            });
        }
        None
    }

    fn match_openbrace(&mut self) -> Option<SyntaxToken> {
        if self.current() == '{' {
            self.next();
            return Some(SyntaxToken {
                text: '{'.to_string(),
                position: self.position - 1,
                kind: SyntaxKind::OpenBraceToken,
            });
        }
        None
    }

    fn match_closebrace(&mut self) -> Option<SyntaxToken> {
        if self.current() == '}' {
            self.next();
            return Some(SyntaxToken {
                text: '}'.to_string(),
                position: self.position - 1,
                kind: SyntaxKind::CloseBraceToken,
            });
        }
        None
    }

    fn match_openbracket(&mut self) -> Option<SyntaxToken> {
        if self.current() == '[' {
            self.next();
            return Some(SyntaxToken {
                text: '['.to_string(),
                position: self.position - 1,
                kind: SyntaxKind::OpenBracketToken,
            });
        }
        None
    }

    fn match_closebracket(&mut self) -> Option<SyntaxToken> {
        if self.current() == ']' {
            self.next();
            return Some(SyntaxToken {
                text: ']'.to_string(),
                position: self.position - 1,
                kind: SyntaxKind::CloseBracketToken,
            });
        }
        None
    }

    fn match_comma(&mut self) -> Option<SyntaxToken> {
        if self.current() == ',' {
            self.next();
            return Some(SyntaxToken {
                text: ','.to_string(),
                position: self.position - 1,
                kind: SyntaxKind::CommaToken,
            });
        }
        None
    }

    fn match_colon(&mut self) -> Option<SyntaxToken> {
        if self.current() == ':' {
            self.next();
            return Some(SyntaxToken {
                text: ':'.to_string(),
                position: self.position - 1,
                kind: SyntaxKind::ColonToken,
            });
        }
        None
    }

    fn match_true(&mut self) -> Option<SyntaxToken> {
        if self.current() == 't' && self.peek(1) == 'r' && self.peek(2) == 'u' && self.peek(3) == 'e' {
            let start = self.position;
            self.skip(4);
            return Some(SyntaxToken {
                text: "true".to_string(),
                position: start,
                kind: SyntaxKind::TrueLiteralToken,
            });
        }
        None
    }

    fn match_false(&mut self) -> Option<SyntaxToken> {
        if self.current() == 'f' && self.peek(1) == 'a' && self.peek(2) == 'l' && self.peek(3) == 's' && self.peek(4) == 'e' {
            let start = self.position;
            self.skip(5);
            return Some(SyntaxToken {
                text: "false".to_string(),
                position: start,
                kind: SyntaxKind::FalseLiteralToken,
            });
        }
        None
    }

    fn match_null(&mut self) -> Option<SyntaxToken> {
        if self.current() == 'n' && self.peek(1) == 'u' && self.peek(2) == 'l' && self.peek(3) == 'l' {
            let start = self.position;
            self.skip(4);
            return Some(SyntaxToken {
                text: "null".to_string(),
                position: start,
                kind: SyntaxKind::NullLiteralToken,
            });
        }
        None
    }

    fn match_number(&mut self) -> Option<SyntaxToken> {
        if self.matches_number() {
            let start = self.position;
            while self.matches_number() {
                self.next()
            }
            return Some(SyntaxToken {
                text: self.text[start..self.position].to_string(),
                position: start,
                kind: SyntaxKind::NumberLiteralToken,
            });
        }
        None
    }

    fn matches_number(&self) -> bool {
        self.current().is_numeric()
            || self.current() == '-'
            || self.current() == '+'
            || self.current() == '.'
            || self.current() == 'e'
            || self.current() == 'E'
    }

    fn match_string(&mut self) -> Option<SyntaxToken> {
        if self.matches_string() {
            let start = self.position;
            self.next();
            loop {
                if self.matches_string() {
                    self.next();
                    break;
                } else {
                    self.next();
                }
            }
            return Some(SyntaxToken {
                text: self.text[start..self.position].to_string(),
                position: start,
                kind: SyntaxKind::StringLiteralToken,
            });
        }
        None
    }

    fn matches_string(&self) -> bool {
        self.current() == '"' || self.current() == '\''
    }

    fn match_identifier(&mut self) -> Option<SyntaxToken> {
        if self.current().is_alphanumeric() {
            let start = self.position;
            while self.current().is_alphanumeric() {
                self.next()
            }
            return Some(SyntaxToken {
                text: self.text[start..self.position].to_string(),
                position: start,
                kind: SyntaxKind::IdentifierToken,
            });
        }
        None
    }

    fn match_comment(&mut self) -> Option<SyntaxToken> {
        if self.matches_comment() {
            let start = self.position;
            loop {
                self.next();
                if self.current() == '\r' || self.current() == '\n' || self.matches_eof() {
                    break;
                }
            }
            return Some(SyntaxToken {
                text: self.text[start..self.position].to_string(),
                position: start,
                kind: SyntaxKind::CommentToken,
            });
        }
        None
    }

    fn matches_comment(&self) -> bool {
        self.current() == '/' && self.peek(1) == '/'
    }

    fn match_blockcomment(&mut self) -> Option<SyntaxToken> {
        if self.matches_opencomment() {
            let start = self.position;
            self.skip(2);
            while !self.matches_closecomment() && !self.matches_eof() {
                self.next();
            }
            if self.matches_closecomment() {
                self.skip(2);
            }
            return Some(SyntaxToken {
                text: self.text[start..self.position].to_string(),
                position: start,
                kind: SyntaxKind::BlockCommentToken,
            });
        }
        None
    }

    fn matches_opencomment(&self) -> bool {
        self.current() == '/' && self.peek(1) == '*'
    }

    fn matches_closecomment(&self) -> bool {
        self.current() == '*' && self.peek(1) == '/'
    }
}
