use crate::syntax_kind::SyntaxKindId;

mod tree_builder;



#[derive(Debug)]
pub enum ParseEvent {
    /// Start of the node. Must be either abandoned or finished by paired Finish event.
    /// If it is abandoned
    Start {
        kind: SyntaxKindId,
        /// Offset in event stream from current position
        forward_parent: Option<u32>
    },
    /// Finish previous `Start` event
    Finish,
    /// Produce leaf node with single token inside
    Token {
        token_type: SyntaxKindId
    },
    Error {
        msg: String
    }
}