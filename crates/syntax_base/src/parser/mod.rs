use crate::tokens::TokenInfo;
use crate::nodes::syntax::SyntaxNode;

pub mod event;
pub mod parser_api;
pub mod parser_impl;

pub trait Parser {
    // TODO abstract over result node type?
    fn parse(&self, text: &str, tokens: Vec<TokenInfo>) -> SyntaxNode;

//     TODO reparse?
//    fn try_reparse() -> Option<>;
}

// TODO actually generalize Parser to ReadonlyTreeNode
//pub trait ReadonlyTreeNode : Sized {
//    fn children(&self) -> Vec<Self>;
//    fn kind(&self) -> SyntaxKind;
//}