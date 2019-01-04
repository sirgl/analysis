use text_unit::TextUnit;
use std::str::Chars;
use syntax_base::syntax_kind::SyntaxKindId;
use syntax_base::lexer::Lexer;
use syntax_base::tokens::TokenInfo;
use crate::simple_lang::syntax::*;

pub struct SimpleLexer {}

impl Lexer for SimpleLexer {
    fn tokenize(&self, text: &str) -> Vec<TokenInfo> {
        tokenize(text)
    }
}

/// Copied from matklad's rust-analyser
struct Ptr<'a> {
    token_len: TextUnit,
    text: &'a str
}

impl<'a> Ptr<'a> {
    pub fn new(text: &'a str) -> Self {
        Ptr { token_len: TextUnit::from(0), text }
    }

    /// Gets the current character, if one exists.
    pub fn current(&self) -> Option<char> {
        self.chars().next()
    }

    /// Checks whether the current character is `c`.
    pub fn at(&self, c: char) -> bool {
        self.current() == Some(c)
    }

    /// Checks whether the next characters match `s`.
    pub fn at_str(&self, s: &str) -> bool {
        let chars = self.chars();
        chars.as_str().starts_with(s)
    }

    /// Returns an iterator over the remaining characters.
    fn chars(&self) -> Chars {
        let len: u32 = self.token_len.into();
        self.text[len as usize..].chars()
    }

    /// Moves to the next character.
    pub fn bump(&mut self) -> Option<char> {
        let ch = self.chars().next()?;
        self.token_len += TextUnit::of_char(ch);
        Some(ch)
    }

    /// Moves to the next character as long as `pred` is satisfied.
    pub fn bump_while<F: Fn(char) -> bool>(&mut self, pred: F) {
        loop {
            match self.current() {
                Some(c) if pred(c) => {
                    self.bump();
                }
                _ => return,
            }
        }
    }

    /// Returns the text up to the current point.
    pub fn current_token_text(&self) -> &str {
        let len: u32 = self.token_len.into();
        &self.text[..len as usize]
    }

    /// Length of the token
    pub fn into_len(self) -> TextUnit {
        self.token_len
    }
}

fn tokenize(text: &str) -> Vec<TokenInfo> {
    let mut text = text;
    let mut acc = Vec::new();
    while !text.is_empty() {
        let token = next_token(text);
        acc.push(token);
        let len: u32 = token.len.into();
        text = &text[len as usize..];
    }
    acc
}

fn next_token(text: &str) -> TokenInfo {
    assert!(!text.is_empty());
    let mut ptr = Ptr::new(text);
    let c = ptr.bump().unwrap();
    let kind = next_token_inner(&mut ptr, c);
    let len = ptr.into_len();
    TokenInfo { len, token_type: kind }
}

fn next_token_inner(p: &mut Ptr, first: char) -> SyntaxKindId {
    if is_whitespace(first) {
        p.bump_while(is_whitespace);
        return WHITESPACE
    }

    if is_digit(first) {
        p.bump_while(is_digit);
        return NUM
    }


    if is_ident_start(first) {
        p.bump_while(is_ident_tail);
        return match p.current_token_text() {
            "fun" => FUN_KW,
            _ => ID,

        }
    }

    match first {
        '{' => LBRACE,
        '}' => RBRACE,
        '(' => LPAR,
        ')' => RPAR,
        ',' => COMMA,
        '+' => PLUS,
        '-' => MINUS,
        '=' => EQ,
        '*' => MUL,
        '/' => DIV,
        _ => SyntaxKindId::ERROR
    }
}

fn is_digit(ch: char) -> bool {
    ch.is_digit(10)
}

fn is_ident_start(ch: char) -> bool {
    ch.is_alphabetic()
}

fn is_ident_tail(ch: char) -> bool {
    is_ident_start(ch) || is_digit(ch)
}

fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\n' || ch == '\t' || ch == '\r'
}


#[cfg(test)]
mod tests {
    use crate::simple_lang::SimpleLangSyntax;
    use syntax_base::syntax::SyntaxDefinition;
    use std::path::PathBuf;
    use crate::test_support::LexerTest;

    #[test]
    fn test1() {
        by_file("a")
    }

    #[test]
    fn test_whitespace() {
        by_file("whitespace")
    }

    const EXT: &str = "simple";
    const BASE_PATH: &str = "./tests/data/simple/lexer";

    fn by_file(name: &str) {
        let syntax = SimpleLangSyntax {};
        let lexer = syntax.lexer().unwrap();
        let test = LexerTest::new(
            &syntax,
            lexer.as_ref(),
            EXT.to_string(),
            PathBuf::from(BASE_PATH)
        );
        test.by_file(name);
    }
}
