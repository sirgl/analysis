use crate::parser::parser_impl::sink::ParseEventSink;
use crate::parser::event::ParseEvent;
use smol_str::SmolStr;
use text_unit::TextRange;
use crate::tokens::TokenInfo;
use text_unit::TextUnit;
use std::mem;
use crate::syntax_kind::SyntaxKindId;

pub(crate) struct TreeBuilder <'a, S : ParseEventSink> {
    sink: S,
    tokens: &'a [TokenInfo],
    events: &'a mut [ParseEvent],
    text: &'a str,
    token_pos: usize,
    text_pos: TextUnit
}

impl<'a, S: ParseEventSink> TreeBuilder<'a, S> {
    pub fn new(
        sink: S,
        tokens: &'a [TokenInfo],
        events: &'a mut [ParseEvent],
        text: &'a str,
        token_pos: usize, // is it required?
    ) -> Self {
        TreeBuilder { sink, tokens, events, text, token_pos, text_pos: TextUnit::from(0) }
    }
}

impl <'a, S : ParseEventSink>TreeBuilder<'a, S> {
    fn build(mut self) -> S {

        for i in 0..self.events.len() {
            let token = &self.tokens[i]; // TODO token position?
            match mem::replace(&mut self.events[i], tombstone()) {
                ParseEvent::Start { kind, forward_parent } => {
                    if forward_parent.is_none() {
                        self.sink.start_internal(kind)
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
                ParseEvent::Error { msg } => {
                    self.sink.error(msg)
                }
            }
            self.text_pos += token.len;
            self.token_pos += 1;
        }
        self.sink
    }

    fn finish(&mut self) -> S::Tree {
        self.sink.finish()
    }

    fn leaf(&mut self, token_type: SyntaxKindId, token_len: TextUnit) {
        let range = TextRange::offset_len(self.text_pos, token_len);
        let token_text: SmolStr = self.text[range].into();
        self.sink.leaf(token_type, token_text);
    }

    fn error() {

    }

//    fn text()
}

fn tombstone() -> ParseEvent {
    ParseEvent::Start {
        kind: SyntaxKindId::TOMBSTONE,
        forward_parent: None,
    }
}