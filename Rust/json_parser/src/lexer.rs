use std::ops::Index;

use crate::token::*;
pub struct Lexer{
    text: String,
    position: usize,
    chars: Vec<char>
}
//TODO: Diagnostics
//TODO: String escape
impl Lexer {
    pub fn new(text: &str) -> Lexer{
        Lexer { text: text.to_string(), position: 0, chars: text.chars().collect() }
    }

    fn current(&self) -> &char{
        self.peek(0)
    }

    fn next(&mut self){
        self.skip(1)
    }

    fn skip(&mut self, offest: usize){
        self.position += offest
    }

    fn peek(&self, offest: usize) -> &char{
        let index = self.position + offest;
        if index >= self.text.len() {
            return &'\0';
        }
        self.chars.index(index)
    }

    pub fn lex(&mut self) -> SyntaxToken{
        if self.current() == &'\0'{
            return SyntaxToken {
                text: "".to_string(),
                position: self.position,
                kind: SyntaxKind::EndOfFileToken
            }
        }
        if self.match_whitespace() {
            let start = self.position;
            while self.match_whitespace() {
                self.next()
            }
            return SyntaxToken {
                text: self.text[start..self.position].to_string(),
                position: start,
                kind: SyntaxKind::WhiteSpaceToken
            }
        }
        if self.match_openbrace(){
            self.next();
            return SyntaxToken {
                text: '{'.to_string(),
                position: self.position - 1,
                kind: SyntaxKind::OpenBraceToken
            }
        }
        if self.match_closebrace(){
            self.next();
            return SyntaxToken {
                text: '}'.to_string(),
                position: self.position - 1,
                kind: SyntaxKind::CloseBraceToken
            }
        }
        if self.match_openbracket(){
            self.next();
            return SyntaxToken {
                text: '['.to_string(),
                position: self.position - 1,
                kind: SyntaxKind::OpenBracketToken
            }
        }
        if self.match_closebracket(){
            self.next();
            return SyntaxToken {
                text: ']'.to_string(),
                position: self.position - 1,
                kind: SyntaxKind::CloseBracketToken
            }
        }
        if self.match_comma(){
            self.next();
            return SyntaxToken {
                text: ','.to_string(),
                position: self.position - 1,
                kind: SyntaxKind::CommaToken
            }
        }
        if self.match_colon(){
            self.next();
            return SyntaxToken {
                text: ':'.to_string(),
                position: self.position - 1,
                kind: SyntaxKind::ColonToken
            }
        }
        if self.match_true(){
            let start = self.position;
            self.skip(4);
            return SyntaxToken {
                text: "true".to_string(),
                position: start,
                kind: SyntaxKind::TrueLiteralToken
            }
        }
        if self.match_false(){
            let start = self.position;
            self.skip(5);
            return SyntaxToken {
                text: "false".to_string(),
                position: start,
                kind: SyntaxKind::FalseLiteralToken
            }
        }
        if self.match_null(){
            let start = self.position;
            self.skip(4);
            return SyntaxToken {
                text: "null".to_string(),
                position: start,
                kind: SyntaxKind::NullLiteralToken
            }
        }
        if self.match_number() {
            let start = self.position;
            while self.match_number() {
                self.next()
            }
            return SyntaxToken {
                text: self.text[start..self.position].to_string(),
                position: start,
                kind: SyntaxKind::NumberLiteralToken
            }
        }
        if self.match_string(){
            let start = self.position;
            self.next();
            loop {
                if self.match_string(){
                    self.next();
                    break;
                }else{
                    self.next();
                }
            }
            return SyntaxToken {
                text: self.text[start..self.position].to_string(),
                position: start,
                kind: SyntaxKind::StringLiteralToken
            }
        }
        if self.match_identifier() {
            let start = self.position;
            while self.match_identifier() {
                self.next()
            }
            return SyntaxToken {
                text: self.text[start..self.position].to_string(),
                position: start,
                kind: SyntaxKind::IdentifierToken
            }
        }
        if self.match_comment() {
            let start = self.position;
            loop {
                self.next();
                if self.current() == &'\r' || self.current() == &'\n' || self.current() == &'\0'{
                    break;
                }
            }
            return SyntaxToken {
                text: self.text[start..self.position].to_string(),
                position: start,
                kind: SyntaxKind::CommentToken
            }
        }
        if self.match_opencomment() {
            let start = self.position;
            self.skip(2);
            while !self.match_closecomment() && self.current() != &'\0'{
                self.next();
            }
            if self.match_closecomment(){
                self.skip(2);
            }
            return SyntaxToken {
                text: self.text[start..self.position].to_string(),
                position: start,
                kind: SyntaxKind::BlockCommentToken
            }
        }
        let bad = SyntaxToken { text: self.current().to_string(), position: self.position, kind: SyntaxKind::BadToken };        
        self.next();
        bad
    }

    fn match_whitespace(&self) -> bool{
        self.current().is_whitespace()
    }

    fn match_openbrace(&self) -> bool{
        self.current() == &'{'
    }

    fn match_closebrace(&self) -> bool{
        self.current() == &'}'
    }

    fn match_openbracket(&self) -> bool{
        self.current() == &'['
    }

    fn match_closebracket(&self) -> bool{
        self.current() == &']'
    }

    fn match_comma(&self) -> bool{
        self.current() == &','
    }

    fn match_colon(&self) -> bool{
        self.current() == &':'
    }

    fn match_true(&self) -> bool{
        self.text[self.position..self.position + 4] == "true".to_string()
    }

    fn match_false(&self) -> bool{
        self.text[self.position..self.position + 5] == "false".to_string()
    }

    fn match_null(&self) -> bool{
        self.text[self.position..self.position + 4] == "null".to_string()
    }

    fn match_number(&self) -> bool{
        self.current().is_numeric() || self.current() == &'-'|| self.current() == &'+'|| self.current() == &'.' || self.current() == &'e' || self.current() == &'E'
    }

    fn match_string(&self) -> bool {
        self.current() == &'"' || self.current() == &'\''
    }
    
    fn match_identifier(&self) -> bool{
        self.current().is_alphanumeric()
    }

    fn match_comment(&self) -> bool{
        self.current() == &'/' && self.peek(1) == &'/'
    }

    fn match_opencomment(&self) -> bool{
        self.current() == &'/' && self.peek(1) == &'*'
    }

    fn match_closecomment(&self) -> bool{
        self.current() == &'*' && self.peek(1) == &'/'
    }
}