use crate::language::LanguageId;
use crate::syntax_kind_set::SyntaxKindSet;

/// Single representation both for token types and node types
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SyntaxKind {
    /// TODO it is pretty bad, that purely syntax crate depends on language
    pub language_id: LanguageId,
    pub syntax_kind_id: SyntaxKindId,
}

impl SyntaxKind {
    pub fn new(language_id: LanguageId, syntax_kind_id: SyntaxKindId) -> Self {
        SyntaxKind { language_id, syntax_kind_id }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SyntaxKindId {
    pub id: u16
}

impl SyntaxKindId {
    pub const fn new(id: u16) -> Self {
        SyntaxKindId { id }
    }
}

impl SyntaxKindId {
    /// 0-4 ids reserved for all languages
    pub const TOMBSTONE: SyntaxKindId = SyntaxKindId { id: 0 };
    pub const END: SyntaxKindId = SyntaxKindId { id: 1 };
    pub const ERROR: SyntaxKindId = SyntaxKindId { id: 2 };
    pub const ROOT: SyntaxKindId = SyntaxKindId { id: 3 };
    pub const INJECTED_ROOT: SyntaxKindId = SyntaxKindId { id: 4 };
}

impl SyntaxKindSet for SyntaxKindId {
    fn matches_by_id(&self, id: SyntaxKindId) -> bool {
        self.id == id.id
    }
}



