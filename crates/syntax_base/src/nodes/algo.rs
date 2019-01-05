pub use rowan::LeafAtOffset;
use crate::nodes::syntax::SyntaxNodeRef;
use text_unit::TextUnit;
use crate::nodes::syntax::SyntaxNode;
use text_unit::TextRange;

pub fn find_leaf_at_offset(node: SyntaxNodeRef, offset: TextUnit) -> LeafAtOffset<SyntaxNodeRef> {
    match node.0.leaf_at_offset(offset) {
        LeafAtOffset::None => LeafAtOffset::None,
        LeafAtOffset::Single(n) => LeafAtOffset::Single(SyntaxNode(n)),
        LeafAtOffset::Between(l, r) => LeafAtOffset::Between(SyntaxNode(l), SyntaxNode(r)),
    }
}

pub fn find_covering_node(root: SyntaxNodeRef, range: TextRange) -> SyntaxNodeRef {
    SyntaxNode(root.0.covering_node(range))
}

pub fn generate<T>(seed: Option<T>, step: impl Fn(&T) -> Option<T>) -> impl Iterator<Item = T> {
    ::itertools::unfold(seed, move |slot| {
        slot.take().map(|curr| {
            *slot = step(&curr);
            curr
        })
    })
}
