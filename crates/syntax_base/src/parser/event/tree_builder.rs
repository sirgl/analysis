use crate::parser::parser_impl::sink::ParseEventSink;
use crate::parser::event::ParseEvent;
use smol_str::SmolStr;
use text_unit::TextRange;
use crate::tokens::TokenInfo;
use text_unit::TextUnit;
use std::mem;
use crate::syntax_kind::SyntaxKindId;
use crate::language::LanguageId;
use crate::syntax_kind::SyntaxKind;

pub struct TreeBuilder <'a, T, S : ParseEventSink<T>> {
    sink: S,
    tokens: &'a [TokenInfo],
    events: &'a mut [ParseEvent],
    text: &'a str,
    token_pos: usize,
    text_pos: TextUnit,
    language_id: LanguageId,
    tree_marker: std::marker::PhantomData<T>,
}

impl<'a, T, S: ParseEventSink<T>> TreeBuilder<'a, T, S> {
    pub fn new(
        sink: S,
        tokens: &'a [TokenInfo],
        events: &'a mut [ParseEvent],
        text: &'a str,
        language_id: LanguageId,
    ) -> Self {
        TreeBuilder { sink, tokens, events, text, language_id, token_pos: 0, text_pos: TextUnit::from(0), tree_marker: std::marker::PhantomData {} }
    }
}

impl<'a, T, S: ParseEventSink<T>> TreeBuilder<'a, T, S> {

    pub fn build(mut self) -> S {
        for i in 0..self.events.len() {
            let token = &self.tokens[i]; // TODO token position?
            match mem::replace(&mut self.events[i], tombstone()) {
                ParseEvent::Start { kind, forward_parent } => {
                    if forward_parent.is_none() {
                        self.sink.start_internal(SyntaxKind::new(self.language_id, kind))
                    } else {
                        panic!();
                    }
                }
                ParseEvent::Finish => {
                    self.finish();
                }
                ParseEvent::Token { token_type } => {
                    self.leaf(token_type, token.len)
                }
                ParseEvent::Error { diagnostic } => {
                    self.sink.error(diagnostic)
                }
            }
            self.text_pos += token.len;
            self.token_pos += 1;
        }
        self.sink
    }

    fn finish(&mut self) {
        self.sink.finish_internal()
    }

    fn leaf(&mut self, token_type: SyntaxKindId, token_len: TextUnit) {
        let range = TextRange::offset_len(self.text_pos, token_len);
        let token_text: SmolStr = self.text[range].into();
        self.sink.leaf(SyntaxKind::new(self.language_id, token_type), token_text);
    }

    fn error(&mut self) {
        unimplemented!()
    }

//    fn text()
}

fn tombstone() -> ParseEvent {
    ParseEvent::Start {
        kind: SyntaxKindId::TOMBSTONE,
        forward_parent: None,
    }
}