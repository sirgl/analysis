pub mod sink;

use crate::tokens::TokenInfo;
use crate::parser::event::ParseEvent;
use crate::syntax_kind::SyntaxKindId;

pub(crate) struct ParserImpl<'a> {
    events: Vec<ParseEvent>,
    input: ParserInput<'a>,
    position: u32,
}


impl<'a> ParserImpl<'a> {
    pub fn new(events: Vec<ParseEvent>, input: ParserInput<'a>, text: &'a str, position: u32) -> Self {
        ParserImpl { events, input, position }
    }

    pub fn into_events(self) -> Vec<ParseEvent> {
        self.events
    }

    /// Start internal node.
    pub fn start(&mut self) -> u32 {
        let pos = self.events.len() as u32;
        self.event(ParseEvent::Start { kind: SyntaxKindId::TOMBSTONE, forward_parent: None });
        pos
    }

    /// Finish internal node
    pub fn finish(&mut self) {
        self.event(ParseEvent::Finish {});
    }

    pub fn error(&mut self, msg: String) {
        self.event(ParseEvent::Error { msg });
    }

    /// nth element's type from current position of parser
    pub fn nth(&self, offset: u32) -> SyntaxKindId {
        self.input.token_type(self.position + offset)
    }

    pub fn bump(&mut self) {
        self.position += 1;
    }

    pub fn leaf(&mut self, token_type: SyntaxKindId) {
        self.event(ParseEvent::Token { token_type });
        self.bump();
    }

    fn event(&mut self, event: ParseEvent) {
        self.events.push(event)
    }
}

pub struct ParserInput<'a> {
    tokens: Vec<TokenInfo>,
    text: &'a str,
}

impl<'a> ParserInput<'a> {
    fn token_type(&self, position: u32) -> SyntaxKindId {
        if position >= self.tokens.len() as u32 {
            SyntaxKindId::END
        } else {
            self.tokens[position as usize].token_type
        }
    }

//    fn text(&self, position: u32) -> SmolStr {
//
//    }
}