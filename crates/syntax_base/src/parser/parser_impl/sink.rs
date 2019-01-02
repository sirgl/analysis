use smol_str::SmolStr;

use crate::syntax_kind::SyntaxKindId;

pub(crate) trait ParseEventSink {
    type Tree;

    fn leaf(&mut self, kind: SyntaxKindId, text: SmolStr);
    fn start_internal(&mut self, kind: SyntaxKindId);
    fn finish_internal(&mut self);
    fn error(&mut self, error_message: String);
    fn finish(&mut self) -> Self::Tree;
}