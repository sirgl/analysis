use rowan::TreeRoot;
use rowan::OwnedRoot;

use rowan::RefRoot;
use crate::simple_lang::syntax::*;
use syntax_base::nodes::syntax::PlatformTypes;
use syntax_base::nodes::syntax::SyntaxNode;
use syntax_base::nodes::syntax::BaseSyntaxNode;
use syntax_base::nodes::syntax::LanguageNodeSet;
use syntax_base::nodes::syntax::SyntaxNodeRef;

pub struct FunNode<R: TreeRoot<PlatformTypes> = OwnedRoot<PlatformTypes>> {
    pub syntax: SyntaxNode<R>,
}

pub type FunNodeRef<'a> = FunNode<RefRoot<'a, PlatformTypes>>;

impl<'a> BaseSyntaxNode<'a> for FunNodeRef<'a> {}
impl<'a> LanguageNodeSet<'a> for FunNodeRef<'a> {
    fn cast(syntax: SyntaxNodeRef<'a>) -> Option<Self> {
        let kind = syntax.0.kind();
        // TODO language_id check?
        match kind.syntax_kind_id {
            FUNCTION => Some(FunNode { syntax }),
            _ => None
        }
    }

    fn syntax(self) -> SyntaxNodeRef<'a> {
        self.syntax
    }
}

pub struct ArgsNode<R: TreeRoot<PlatformTypes> = OwnedRoot<PlatformTypes>> {
    pub syntax: SyntaxNode<R>,
}
pub type ArgsNodeRef<'a> = ArgsNode<RefRoot<'a, PlatformTypes>>;

impl<'a> BaseSyntaxNode<'a> for ArgsNodeRef<'a> {}
impl<'a> LanguageNodeSet<'a> for ArgsNodeRef<'a> {
    fn cast(syntax: SyntaxNodeRef<'a>) -> Option<Self> {
        let kind = syntax.0.kind();
        // TODO language_id check?
        match kind.syntax_kind_id {
            ARGS => Some(ArgsNode { syntax }),
            _ => None
        }
    }

    fn syntax(self) -> SyntaxNodeRef<'a> {
        self.syntax
    }
}

pub struct BlockNode<R: TreeRoot<PlatformTypes> = OwnedRoot<PlatformTypes>> {
    pub syntax: SyntaxNode<R>,
}
pub type BlockNodeRef<'a> = BlockNode<RefRoot<'a, PlatformTypes>>;
impl<'a> BaseSyntaxNode<'a> for BlockNodeRef<'a> {}
impl<'a> LanguageNodeSet<'a> for BlockNodeRef<'a> {
    fn cast(syntax: SyntaxNodeRef<'a>) -> Option<Self> {
        let kind = syntax.0.kind();
        // TODO language_id check?
        match kind.syntax_kind_id {
            BLOCK => Some(BlockNode { syntax }),
            _ => None
        }
    }

    fn syntax(self) -> SyntaxNodeRef<'a> {
        self.syntax
    }
}

pub struct ParameterNode<R: TreeRoot<PlatformTypes> = OwnedRoot<PlatformTypes>> {
    pub syntax: SyntaxNode<R>,
}
pub type ParameterNodeRef<'a> = ParameterNode<RefRoot<'a, PlatformTypes>>;
impl<'a> BaseSyntaxNode<'a> for ParameterNodeRef<'a> {}
impl<'a> LanguageNodeSet<'a> for ParameterNodeRef<'a> {
    fn cast(syntax: SyntaxNodeRef<'a>) -> Option<Self> {
        let kind = syntax.0.kind();
        // TODO language_id check?
        match kind.syntax_kind_id {
            BLOCK => Some(ParameterNode { syntax }),
            _ => None
        }
    }

    fn syntax(self) -> SyntaxNodeRef<'a> {
        self.syntax
    }
}

pub struct FileNode<R: TreeRoot<PlatformTypes> = OwnedRoot<PlatformTypes>> {
    pub syntax: SyntaxNode<R>,
}
pub type FileNodeRef<'a> = FileNode<RefRoot<'a, PlatformTypes>>;
impl<'a> BaseSyntaxNode<'a> for FileNodeRef<'a> {}
impl<'a> LanguageNodeSet<'a> for FileNodeRef<'a> {
    fn cast(syntax: SyntaxNodeRef<'a>) -> Option<Self> {
        let kind = syntax.0.kind();
        // TODO language_id check?
        match kind.syntax_kind_id {
            BLOCK => Some(FileNode { syntax }),
            _ => None
        }
    }

    fn syntax(self) -> SyntaxNodeRef<'a> {
        self.syntax
    }
}
