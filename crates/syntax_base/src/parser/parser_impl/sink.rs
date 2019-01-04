use smol_str::SmolStr;

use crate::syntax_kind::SyntaxKindId;
use rowan::GreenNodeBuilder;
use rowan::Types;
use errors::TextDiagnostic;
use rowan::GreenNode;

pub trait ParseEventSink<T> {
    fn leaf(&mut self, kind: SyntaxKindId, text: SmolStr);
    fn start_internal(&mut self, kind: SyntaxKindId);
    fn finish_internal(&mut self);
    fn error(&mut self, error: TextDiagnostic);
    fn finish(self) -> T;
}

pub struct GreenTreeEventSink<TY : Types> {
    errors: Vec<TextDiagnostic>,
    inner: GreenNodeBuilder<TY>,
}

impl<TY: Types> GreenTreeEventSink<TY> {
    pub fn new() -> Self {
        GreenTreeEventSink { errors: Vec::new(), inner: GreenNodeBuilder::new() }
    }
}

impl <T: Types> ParseEventSink<GreenNode<T>> for GreenTreeEventSink<T> {
    fn leaf(&mut self, kind: SyntaxKindId, text: SmolStr) {
//        self.inner.
        unimplemented!()
    }

    fn start_internal(&mut self, kind: SyntaxKindId) {
//        self.inner.start_internal(kind)
        unimplemented!()
    }

    fn finish_internal(&mut self) {
        unimplemented!()
    }

    fn error(&mut self, error_message: TextDiagnostic) {
        unimplemented!()
    }

    fn finish(self) -> GreenNode<T> {
        self.inner.finish()
    }
}