#![feature(test)]

extern crate test;
extern crate java_syntax;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use java_syntax::*;
use logos::Logos;
use logos::Lexer;


#[cfg(test)]
mod tests {
    use super::*;
    use java_syntax::lexer::JavaTokenType::*;
    use java_syntax::lexer::JavaTokenType;

    #[test]
    fn test_int() {
        check_single("12", IntegerLiteral);
    }

    #[test]
    fn test_long_literal() {
        check_single("43l", IntegerLiteral);
    }

    #[test]
    fn test_long_literal_with_underscore() {
        check_single("11_23_2l", IntegerLiteral);
    }

    #[test]
    fn test_id() {
        check_single("foo", Identifier);
    }

    #[test]
    fn test_id_digits() {
        check_single("foo33", Identifier);
    }

    #[test]
    fn test_keyword() {
        check_single("break", BreakKw);
    }
    #[test]
    fn test_id_like_keyword() {
        check_single("breaking", Identifier);
    }

//    ERRORs should be united into single token
//    #[test]
//    fn test_errors() {
//        check_tokenize("####", vec![]);
//    }

    fn check_single(text: &str, token_type: JavaTokenType) {
        let mut lexer = JavaTokenType::lexer(text);
        let iterator = iter_for_lexer(lexer);
        let tokens: Vec<TokenInfo> = iterator.collect();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, token_type);
    }

    fn check_tokenize(text: &str, expected: Vec<TokenInfo>) {
        let mut lexer: Lexer<JavaTokenType, &str> = JavaTokenType::lexer(text);
        let iterator = iter_for_lexer(lexer);
        let tokens: Vec<TokenInfo> = iterator.collect();
        assert_eq!(expected, tokens);
    }

//    fn read_file() -> String {
//        let mut f = File::open("./benches/Main.java").unwrap();
//        let mut contents = String::new();
//        f.read_to_string(&mut contents).unwrap();
//        contents
//    }
}