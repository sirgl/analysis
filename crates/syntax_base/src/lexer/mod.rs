use crate::tokens::TokenInfo;

// TODO replace with function pointer?
pub trait Lexer {
    fn tokenize(&self, text: &str) -> Vec<TokenInfo>;
}