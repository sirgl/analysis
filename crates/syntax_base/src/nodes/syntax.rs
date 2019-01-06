use crate::language::LanguageId;
use crate::syntax_kind_set::IterableSyntaxKindSet;
use rowan::WalkEvent;
use rowan::TreeRoot;
use smol_str::SmolStr;
use errors::TextDiagnostic;
use crate::syntax_kind::SyntaxKind;
use text_unit::TextRange;
use crate::nodes::syntax_text::SyntaxText;
use core::fmt;
use std::fmt::Formatter;
use std::fmt::Error;
use crate::syntax::SyntaxDefinition;

pub struct PlatformTypes {}

impl rowan::Types for PlatformTypes {
    type Kind = SyntaxKind;
    type RootData = PlatformRootData;
}

#[derive(Debug)]
pub struct PlatformRootData {
    errors: Vec<TextDiagnostic>
}

impl PlatformRootData {
    pub fn new() -> Self {
        PlatformRootData { errors: Vec::new() }
    }
}

pub type OwnedRoot = ::rowan::OwnedRoot<PlatformTypes>;

pub type RefRoot<'a> = ::rowan::RefRoot<'a, PlatformTypes>;

pub type GreenNode = ::rowan::GreenNode<PlatformTypes>;

#[derive(Clone, Copy)]
pub struct SyntaxNode<R: TreeRoot<PlatformTypes> = OwnedRoot>(
    pub rowan::SyntaxNode<PlatformTypes, R>
);
pub type SyntaxNodeRef<'a> = SyntaxNode<RefRoot<'a>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Next,
    Prev,
}

impl<'a> SyntaxNode<RefRoot<'a>> {
    pub fn leaf_text(self) -> Option<&'a SmolStr> {
        self.0.leaf_text()
    }

    pub fn ancestors(self) -> impl Iterator<Item = SyntaxNodeRef<'a>> {
        crate::nodes::algo::generate(Some(self), |&node| node.parent())
    }

    /// Emits descendants in preorder
    pub fn descendants(self) -> impl Iterator<Item = SyntaxNodeRef<'a>> {
        self.preorder().filter_map(|event| match event {
            WalkEvent::Enter(node) => Some(node),
            WalkEvent::Leave(_) => None,
        })
    }

    pub fn siblings(self, direction: Direction) -> impl Iterator<Item = SyntaxNodeRef<'a>> {
        crate::nodes::algo::generate(Some(self), move |&node| match direction {
            Direction::Next => node.next_sibling(),
            Direction::Prev => node.prev_sibling(),
        })
    }

    pub fn preorder(self) -> impl Iterator<Item = WalkEvent<SyntaxNode<RefRoot<'a>>>> {
        self.0.preorder().map(|event| match event {
            WalkEvent::Enter(n) => WalkEvent::Enter(SyntaxNode(n)),
            WalkEvent::Leave(n) => WalkEvent::Leave(SyntaxNode(n)),
        })
    }
}

impl<R: TreeRoot<PlatformTypes>> SyntaxNode<R> {
    pub fn root_data(&self) -> &Vec<TextDiagnostic> {
        &self.0.root_data().errors
    }
    // TODO probably bad to replace with green node!
    pub fn replace_with(&self, replacement: GreenNode) -> GreenNode {
        self.0.replace_with(replacement)
    }
    pub fn borrowed<'a>(&'a self) -> SyntaxNode<RefRoot<'a>> {
        SyntaxNode(self.0.borrowed())
    }
    pub fn owned(&self) -> SyntaxNode<OwnedRoot> {
        SyntaxNode(self.0.owned())
    }
    pub fn kind(&self) -> SyntaxKind {
        self.0.kind()
    }
    pub fn range(&self) -> TextRange {
        self.0.range()
    }
    pub fn text(&self) -> SyntaxText {
        SyntaxText::new(self.borrowed())
    }
    pub fn is_leaf(&self) -> bool {
        self.0.is_leaf()
    }
    pub fn parent(&self) -> Option<SyntaxNode<R>> {
        self.0.parent().map(SyntaxNode)
    }
    pub fn first_child(&self) -> Option<SyntaxNode<R>> {
        self.0.first_child().map(SyntaxNode)
    }
    pub fn last_child(&self) -> Option<SyntaxNode<R>> {
        self.0.last_child().map(SyntaxNode)
    }
    pub fn next_sibling(&self) -> Option<SyntaxNode<R>> {
        self.0.next_sibling().map(SyntaxNode)
    }
    pub fn prev_sibling(&self) -> Option<SyntaxNode<R>> {
        self.0.prev_sibling().map(SyntaxNode)
    }
    pub fn children(&self) -> SyntaxNodeChildren<R> {
        SyntaxNodeChildren(self.0.children())
    }

    pub fn debug_text(&self, def: &SyntaxDefinition) -> String {
        let mut buf = String::new();
        let name = def._syntax_kind_info(self.kind().syntax_kind_id).name;
        buf.push_str(name);
        buf.push_str("@");
        buf.push_str(self.text().to_string().as_str());
        buf
    }
}

pub struct SyntaxNodeChildren<R: TreeRoot<PlatformTypes>>(::rowan::SyntaxNodeChildren<PlatformTypes, R>);

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

impl<R: TreeRoot<PlatformTypes>> fmt::Debug for SyntaxNode<R> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}@{}", self.kind().syntax_kind_id.id, self.text())
    }
}