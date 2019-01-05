use errors::TextDiagnostic;
use rowan::TreeRoot;
use crate::language::LanguageId;
use crate::syntax_kind_set::IterableSyntaxKindSet;
use crate::syntax_kind::SyntaxKind;

pub struct PlatformTypes {}

impl rowan::Types for PlatformTypes {
    type Kind = SyntaxKind;
    type RootData = PlatformRootData;
}

#[derive(Debug)]
pub struct PlatformRootData {
    errors: Vec<TextDiagnostic>
}

pub type OwnedRoot = ::rowan::OwnedRoot<PlatformTypes>;

pub type RefRoot<'a> = ::rowan::RefRoot<'a, PlatformTypes>;


pub struct SyntaxNode<R: TreeRoot<PlatformTypes>>(
    pub rowan::SyntaxNode<PlatformTypes, R>
);
pub type SyntaxNodeRef<'a> = SyntaxNode<RefRoot<'a>>;

/// Base syntax node in platform
pub trait BaseSyntaxNode<'a> : LanguageNodeSet<'a> {}

/// Any trait, that related to some node kinds should implement this
pub trait LanguageNodeSet<'a> {
    fn cast(syntax: SyntaxNodeRef<'a>) -> Option<Self>
        where
            Self: Sized;

    fn syntax(self) -> SyntaxNodeRef<'a>;
}


// TODO use it in syntax description to simplify work for visitor
pub struct LanguageNodeSetDescriptor {
    pub language_id: LanguageId,
    pub implementor_kinds: IterableSyntaxKindSet
}