extern crate logos;

pub mod lexer;
pub mod parser;
pub mod parse_errors;

use logos::Logos;
use logos::Lexer;
use crate::lexer::JavaTokenType;

#[derive(Debug, Eq, PartialEq)]
pub struct TokenInfo {
    pub token: JavaTokenType,
    pub start_offset: usize,
    pub len: usize,
}

pub struct LexemeIterator <'a> {
    lexer: Lexer<JavaTokenType, &'a str>
}

impl <'a> Iterator for LexemeIterator<'a> {
    type Item = TokenInfo;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.lexer.token == JavaTokenType::End {
            return None
        }
        let info = TokenInfo {
            token: self.lexer.token.clone(),
            start_offset: self.lexer.range().start,
            len: self.lexer.range().end - self.lexer.range().start
        };
        self.lexer.advance();
        Some(info)
    }
}

pub fn iter_for_lexer(lexer: Lexer<JavaTokenType, &str>) -> LexemeIterator {
    LexemeIterator { lexer }
}




