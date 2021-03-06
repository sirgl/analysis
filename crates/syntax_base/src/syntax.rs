use crate::lexer::Lexer;
use crate::syntax_kind::SyntaxKindId;
use crate::parser::Parser;

pub struct SyntaxInfo {
    pub name: &'static str
}

/// Parser and lexer may have settings depending on something (e. g. language version)
pub trait SyntaxDefinition {
    fn lexer(&self) -> Option<Box<dyn Lexer>>;

    fn parser<'a>(&self) -> Option<Box<Parser>>; // parser type parameters?

    /// usually autogenerated
    ///
    /// must not handle reserved ids (0-4)
    ///
    /// should not be used outside, should be used syntax_kind_info instead
    fn _syntax_kind_info(&self, id: SyntaxKindId) -> &SyntaxInfo;

    fn id_by_syntax_kind_name(&self, name: &str) -> Option<SyntaxKindId>;

    fn is_trivia(&self, id: SyntaxKindId) -> bool;
}

impl SyntaxDefinition {
    // TODO actually use it
    pub fn syntax_kind_info<'a, 'b>(&'b self, id: SyntaxKindId) -> &'a SyntaxInfo {
        match id {
            SyntaxKindId::TOMBSTONE => &SyntaxInfo { name: "TOMBSTONE" },
            SyntaxKindId::END => &SyntaxInfo { name: "END" },
            SyntaxKindId::ERROR => &SyntaxInfo { name: "ERROR" },
            SyntaxKindId::ROOT => &SyntaxInfo { name: "ROOT" },
            SyntaxKindId::INJECTED_ROOT => &SyntaxInfo { name: "INJECTED_ROOT" },
            _=> panic!("")
//            _ => self._syntax_kind_info(id)
        }
    }
}