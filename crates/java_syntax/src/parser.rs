use rowan::*;
use crate::TokenInfo;


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum SyntaxKind {
    IntLiteral,
    FloatLiteral,
    BinaryExpression,
    Class,
    Function,
}

trait AstNode {

}

enum JavaTypes {}
impl rowan::Types for JavaTypes {
    /// Each node will store a `Kind`.
    type Kind = SyntaxKind;
    /// Errors
    type RootData = Vec<String>;
}

struct JavaParser {
//    parser: ParserApi
}

impl JavaParser {
    fn parse_expr(&mut self) {

    }

    fn parse_file(&mut self) {
//        self.parser.
    }
}

//pub struct JavaParser<'a> {
//    tokens: Vec<TokenInfo>,
//    text: &'a str
//}
//
//impl<'a> JavaParser<'a> {
//    fn parse() {
//
//    }
//}
//
//trait SourceFile {
//
//}