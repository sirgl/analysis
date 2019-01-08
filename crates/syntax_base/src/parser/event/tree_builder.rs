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
use errors::TextDiagnostic;
use errors::Location;

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
        for e in self.events.iter() {
            eprintln!("e = {:?}", e);
        }
        let mut token_pos = 0;
        let mut node_start: usize = 0;
        let mut node_end: usize = 0;
        let mut node_index: usize = 0;
        for i in 0..self.events.len() {
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
                ParseEvent::Token { token_type, is_trivia } => {
                    let token = &self.tokens[token_pos];
                    if is_trivia {

                    } else {
                        self.leaf(token_type, token.len);
                    }
                    token_pos += 1;
                    self.text_pos += token.len;
                }
                ParseEvent::Error { diagnostic } => {
                    self.sink.error(TextDiagnostic::new(
                        diagnostic,
                        Location::Offset(self.text_pos)
                    ))
                }
            }
        }
        self.sink
    }

    fn finish(&mut self) {
        self.sink.finish_internal()
    }

    fn leaf(&mut self, token_type: SyntaxKindId, token_len: TextUnit) {
        let range = TextRange::offset_len(self.text_pos, token_len);
        let token_text: SmolStr = self.text[range].into();
        eprintln!("range = {:?}", range);
        eprintln!("token_text = {:?}", token_text);
        self.sink.leaf(SyntaxKind::new(self.language_id, token_type), token_text);
    }

//    fn text()
}

fn tombstone() -> ParseEvent {
    ParseEvent::Start {
        kind: SyntaxKindId::TOMBSTONE,
        forward_parent: None,
    }
}