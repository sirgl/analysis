pub mod language;
pub mod lexer;
pub mod tokens;
pub mod parser;
pub mod syntax_kind;
pub mod syntax_kind_set;
pub mod syntax;
pub mod nodes;

pub fn escape_str(text: &str) -> String {
    let mut buffer = String::new();
    for ch in text.chars() {
        match ch {
            '\n' => buffer.push_str("\\n"),
            '\t' => buffer.push_str("\\t"),
            _ => buffer.push(ch)
        }
    }
    buffer
}