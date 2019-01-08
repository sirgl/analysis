pub mod sink;

use crate::tokens::TokenInfo;
use crate::parser::event::ParseEvent;
use crate::syntax_kind::SyntaxKindId;
use crate::parser::parser_impl::sink::ParseEventSink;
use crate::parser::event::tree_builder::TreeBuilder;
use crate::parser::parser_impl::sink::GreenTreeEventSink;
use errors::TextDiagnostic;
use crate::language::LanguageId;
use crate::parser::parser_api::TriviaHandler;

pub(crate) struct ParserImpl<'a, T: TriviaHandler> {
    events: Vec<ParseEvent>,
    input: ParserInput<'a>,
    position: u32,
    language_id: LanguageId,
    trivia_handler: &'a TriviaHandler
}


impl<'a, T: TriviaHandler> ParserImpl<'a, T> {
    pub fn new(tokens: Vec<TokenInfo>, text: &'a str, language_id: LanguageId, trivia_handler: &'a T) -> Self {
        ParserImpl { events: vec![], input: ParserInput { text, tokens }, position: 0, language_id, trivia_handler }
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
    pub fn finish(&mut self, pos: u32, kind: SyntaxKindId) {
        match self.events[pos as usize] {
            ParseEvent::Start {
                kind: ref mut slot, ..
            } => {
                *slot = kind;
            }
            _ => unreachable!(),
        }
        self.event(ParseEvent::Finish {});
    }

    pub fn error(&mut self, error: String) {
        self.event(ParseEvent::Error { diagnostic: error });
    }

    /// nth element's type from current position of parser
    pub fn nth(&self, offset: u32) -> SyntaxKindId {
        self.input.token_type(self.position + offset)
    }

    // TODO bump must skip whitespaces and other trivias
    pub fn bump(&mut self) {
        self.position += 1;
        while self.trivia_handler.is_trivia(self.nth(0)) {
            let id = self.nth(0);
            self.trivia(id)
        }
        self.position += 1;
    }

    pub fn trivia(&mut self, token_type: SyntaxKindId) {
        self.position += 1;
        self.event(ParseEvent::Trivia { token_type })
    }

    pub fn leaf(&mut self, token_type: SyntaxKindId) {
        self.event(ParseEvent::Token { token_type });
        self.bump();
    }

    fn event(&mut self, event: ParseEvent) {
        self.events.push(event)
    }

    pub fn build<T, S: ParseEventSink<T>>(mut self, sink: S) -> T {
        let builder: TreeBuilder<T, S> = TreeBuilder::new(
            sink,
            &self.input.tokens,
            &mut self.events,
            self.input.text,
            self.language_id
        );
        builder.build().finish()
    }
}

pub struct Tree {}

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
}