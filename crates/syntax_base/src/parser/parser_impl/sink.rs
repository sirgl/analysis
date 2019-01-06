use smol_str::SmolStr;

use crate::syntax_kind::SyntaxKindId;
use rowan::GreenNodeBuilder;
use rowan::Types;
use errors::TextDiagnostic;
use rowan::GreenNode;
use crate::nodes::syntax::PlatformTypes;
use crate::syntax_kind::SyntaxKind;

pub trait ParseEventSink<T> {
    fn leaf(&mut self, kind: SyntaxKind, text: SmolStr);
    fn start_internal(&mut self, kind: SyntaxKind);
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

impl ParseEventSink<GreenNode<PlatformTypes>> for GreenTreeEventSink<PlatformTypes> {
    fn leaf(&mut self, kind: SyntaxKind, text: SmolStr) {
        self.inner.leaf(kind, text);
    }

    fn start_internal(&mut self, kind: SyntaxKind) {
        self.inner.start_internal(kind);
    }

    fn finish_internal(&mut self) {
        self.inner.finish_internal();
    }

    fn error(&mut self, error_message: TextDiagnostic) {
        self.errors.push(error_message);
    }

    fn finish(self) -> GreenNode<PlatformTypes> {
        self.inner.finish()
    }
}