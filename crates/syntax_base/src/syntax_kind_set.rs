use crate::syntax_kind::SyntaxKindId;

pub trait SyntaxKindSet {
    fn matches_by_id(&self, id: SyntaxKindId) -> bool;
}

/// Small token set capable to store at most 128 syntax kinds
/// Very fast (O(1) all operations)
pub struct SmallSyntaxKindSet {
    pub(crate) bitmask: u128
}

impl SmallSyntaxKindSet {
    pub const fn from_mask(mask: u128) -> Self {
        SmallSyntaxKindSet { bitmask: mask }
    }

    pub const fn from_id(id: SyntaxKindId) -> Self {
        SmallSyntaxKindSet::from_mask(SmallSyntaxKindSet::bitmask(id))
    }

    pub const fn unite_with_id(&self, id: SyntaxKindId) -> Self {
        SmallSyntaxKindSet { bitmask: self.bitmask | SmallSyntaxKindSet::bitmask(id) }
    }

    pub const fn unite_with(&self, set: SmallSyntaxKindSet) -> Self {
        SmallSyntaxKindSet { bitmask: self.bitmask | set.bitmask }
    }

    const fn bitmask(id: SyntaxKindId) -> u128 {
        1u128 << id.id
    }
}


impl SyntaxKindSet for SmallSyntaxKindSet {
    fn matches_by_id(&self, id: SyntaxKindId) -> bool {
        let shifted = 1u128 << id.id;
        self.bitmask & shifted != 0
    }
}

#[cfg(test)]
mod test {
    use crate::syntax_kind::SyntaxKindId;
    use crate::syntax_kind_set::SmallSyntaxKindSet;
    use crate::syntax_kind_set::SyntaxKindSet;

    const C1: SyntaxKindId = SyntaxKindId { id: 0 };
    const C2: SyntaxKindId = SyntaxKindId { id: 1 };

    const S1: SmallSyntaxKindSet = SmallSyntaxKindSet::from_id(C1);
    const S2: SmallSyntaxKindSet = SmallSyntaxKindSet::from_id(C1).unite_with_id(C2);

    #[test]
    fn test_single_match() {
        let id1 = SyntaxKindId { id: 0 };
        let id2 = SyntaxKindId { id: 1 };
        assert!(S1.matches_by_id(id1));
        assert!(!S1.matches_by_id(id2));
    }

    #[test]
    fn test_complex_set_match() {
        let id1 = SyntaxKindId { id: 0 };
        let id2 = SyntaxKindId { id: 1 };
        let id3 = SyntaxKindId { id: 2 };
        assert!(S2.matches_by_id(id1));
        assert!(S2.matches_by_id(id2));
        assert!(!S2.matches_by_id(id3));
    }
}