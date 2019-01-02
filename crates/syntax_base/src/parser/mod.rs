use crate::tokens::TokenInfo;

pub mod event;
pub mod parser_api;
pub mod parser_impl;

pub trait Parser {
    fn parse(&self, text: &str, tokens: Vec<TokenInfo>); // TODO result

//     TODO reparse?
//    fn try_reparse() -> Option<>;
}