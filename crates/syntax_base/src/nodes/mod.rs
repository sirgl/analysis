use errors::TextDiagnostic;
use crate::syntax_kind::SyntaxKindId;
use rowan::TreeRoot;
use rowan::OwnedRoot;
use rowan::RefRoot;

pub struct PlatformTypes {}

impl rowan::Types for PlatformTypes {
    type Kind = SyntaxKindId;
    type RootData = PlatformRootData;
}

#[derive(Debug)]
pub struct PlatformRootData {
    errors: Vec<TextDiagnostic>
}

pub struct SyntaxNode<R: TreeRoot<PlatformTypes> = OwnedRoot<PlatformTypes>>(pub(crate) ::rowan::SyntaxNode<PlatformTypes, R>);
pub type SyntaxNodeRef<'a> = SyntaxNode<RefRoot<'a, PlatformTypes>>;

/// Base syntax node in platform
trait BaseSyntaxNode<'a> {
    fn cast(syntax: SyntaxNodeRef<'a>) -> Option<Self>
        where
            Self: Sized;
    fn syntax(self) -> SyntaxNodeRef<'a>;
}

