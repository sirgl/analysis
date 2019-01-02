extern crate test;

mod lexer;
mod test_support;
mod parser;
mod syntax;

use syntax_base::syntax_kind::*;
use syntax_base::syntax::SyntaxDefinition;
use syntax_base::lexer::Lexer;
use syntax_base::parser::Parser;
use syntax_base::syntax::SyntaxInfo;
use syntax_base::tokens::TokenInfo;
use crate::simple_lang::lexer::SimpleLexer;
use crate::simple_lang::syntax::infos;

pub const LBRACE: SyntaxKindId = SyntaxKindId::new(5);
pub const RBRACE: SyntaxKindId = SyntaxKindId::new(6);
pub const LPAR: SyntaxKindId = SyntaxKindId::new(7);
pub const RPAR: SyntaxKindId = SyntaxKindId::new(8);
pub const COMMA: SyntaxKindId = SyntaxKindId::new(9);
pub const PLUS: SyntaxKindId = SyntaxKindId::new(10);
pub const MINUS: SyntaxKindId = SyntaxKindId::new(11);
pub const MUL: SyntaxKindId = SyntaxKindId::new(12);
pub const DIV: SyntaxKindId = SyntaxKindId::new(13);
pub const EQ: SyntaxKindId = SyntaxKindId::new(14);
pub const SEMI: SyntaxKindId = SyntaxKindId::new(15);
pub const FUN_KW: SyntaxKindId = SyntaxKindId::new(16);
pub const ID: SyntaxKindId = SyntaxKindId::new(17);
pub const NUM: SyntaxKindId = SyntaxKindId::new(18);
pub const WHITESPACE: SyntaxKindId = SyntaxKindId::new(19);
pub const ARGS: SyntaxKindId = SyntaxKindId::new(20);
pub const FUNCTION: SyntaxKindId = SyntaxKindId::new(21);



pub struct SimpleLangSyntax {}

impl SyntaxDefinition for SimpleLangSyntax {
    fn lexer(&self) -> Option<Box<Lexer>> {
        Some(Box::new(SimpleLexer {}))
    }

    fn parser<'a>(&self) -> Option<Box<Parser>> {
        Some(Box::new(SimpleParser {}))
    }


    fn _syntax_kind_info(&self, id: SyntaxKindId) -> &SyntaxInfo {
        match id {
            SyntaxKindId::TOMBSTONE => &SyntaxInfo { name: "TOMBSTONE" },
            SyntaxKindId::END => &SyntaxInfo { name: "END" },
            SyntaxKindId::ERROR => &SyntaxInfo { name: "ERROR" },
            SyntaxKindId::ROOT => &SyntaxInfo { name: "ROOT" },
            SyntaxKindId::INJECTED_ROOT => &SyntaxInfo { name: "INJECTED_ROOT" },
            _ => infos(id)
        }
    }

    fn id_by_syntax_kind_name(&self, name: &str) -> Option<SyntaxKindId> {
        None
    }
}




struct SimpleParser {}

impl Parser for SimpleParser {
    fn parse(&self, text: &str, tokens: Vec<TokenInfo>) {
        unimplemented!()
    }
}







#[cfg(test)]
mod tests {
//    use super::*;
//




    use std::path::PathBuf;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_int() {

    }

    #[test]
    fn test() {
        let cur = PathBuf::from(".");
        eprintln!("cur.canonicalize() = {:?}", cur.canonicalize().unwrap());
        let mut file = File::open(cur).unwrap();

        let mut string = String::new();
        file.read_to_string(&mut string).unwrap();
        eprintln!("string = {:?}", string);
    }
//
//    #[test]
//    fn test_long_literal() {
//        check_single("43l", IntegerLiteral);
//    }
//
//    #[test]
//    fn test_long_literal_with_underscore() {
//        check_single("11_23_2l", IntegerLiteral);
//    }
//
//    #[test]
//    fn test_id() {
//        check_single("foo", Identifier);
//    }
//
//    #[test]
//    fn test_id_digits() {
//        check_single("foo33", Identifier);
//    }
//
//    #[test]
//    fn test_keyword() {
//        check_single("break", BreakKw);
//    }
//    #[test]
//    fn test_id_like_keyword() {
//        check_single("breaking", Identifier);
//    }
//
//    fn check_single(text: &str, token_type: JavaTokenType) {
//        let mut lexer = JavaTokenType::lexer(text);
//        let iterator = iter_for_lexer(lexer);
//        let tokens: Vec<TokenInfo> = iterator.collect();
//        assert_eq!(tokens.len(), 1);
//        assert_eq!(tokens[0].token, token_type);
//    }
//
//    fn check_tokenize(mut lexer: Lexer<JavaTokenType, &str>, expected: Vec<TokenInfo>) {
//        let iterator = iter_for_lexer(lexer);
//        let tokens: Vec<TokenInfo> = iterator.collect();
//        assert_eq!(expected, tokens);
//    }

//    fn read_file() -> String {
//        let mut f = File::open("./benches/Main.java").unwrap();
//        let mut contents = String::new();
//        f.read_to_string(&mut contents).unwrap();
//        contents
//    }
}