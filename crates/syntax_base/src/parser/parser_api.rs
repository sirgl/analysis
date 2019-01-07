use crate::parser::parser_impl::ParserImpl;
use drop_bomb::DropBomb;
use crate::syntax_kind::SyntaxKindId;
use crate::syntax_kind_set::SyntaxKindSet;
use crate::tokens::TokenInfo;
use crate::parser::parser_impl::sink::ParseEventSink;
use crate::language::LanguageId;


pub struct ParserApi<'a>(ParserImpl<'a>);

impl <'a> ParserApi<'a> {
    pub fn new(tokens: Vec<TokenInfo>, text: &'a str, language_id: LanguageId) -> Self {
        ParserApi(ParserImpl::new(tokens, text, language_id))
    }

    // TODO bump must skip whitespaces and other trivias
    /// Increments position in token stream
    pub fn bump(&mut self) {
        self.0.bump();
    }

    /// Returns true, if at current position token with given kind
    pub fn at<T: SyntaxKindSet>(&mut self, set: T) -> bool {
        set.matches_by_id(self.0.nth(0))
    }

    pub fn start(&mut self) -> Marker {
        Marker::new(self.0.start())
    }

    pub fn error<T: Into<String>>(&mut self, message: T) {
        self.0.error(message.into())
    }

    /// Consume the next token if it is `kind`.
    pub fn eat(&mut self, kind: SyntaxKindId) -> bool {
        if !self.at(kind) {
            return false;
        }
        self.bump();
        true
    }

    pub fn err_recover<S: SyntaxKindSet>(&mut self, message: &str, recover_until: S) {
        if self.at(recover_until) {
            self.error(message);
        } else {
            let m = self.start();
            self.error(message);
            self.leaf(self.0.nth(0));
            self.bump();
            m.complete(self, SyntaxKindId::ERROR);
        }
    }

    pub fn leaf(&mut self, token_type: SyntaxKindId) {
        self.0.leaf(token_type)
    }

    pub fn build<T, S: ParseEventSink<T>>(self, sink: S) -> T {
        self.0.build(sink)
    }
}

pub struct Marker {
    position: u32,
    bomb: DropBomb
}

impl Marker {
    pub fn new(position: u32) -> Self {
        Marker { position, bomb: DropBomb::new("Every start operation must have paired finish operation") }
    }
}

impl Marker {
    pub fn complete(mut self, p: &mut ParserApi, syntax_kind: SyntaxKindId) -> CompletedMarker {
        self.bomb.defuse();
        p.0.finish(self.position, syntax_kind);
        CompletedMarker::new(self.position, syntax_kind)
    }

    pub fn abandon(mut self, p: &mut ParserApi) {
        // TODO
        unimplemented!();
//        self.bomb.defuse();
//        p.0.
    }
}

pub struct CompletedMarker {
    position: u32,
    syntax_kind: SyntaxKindId
}

impl CompletedMarker {
    pub fn new(position: u32, syntax_kind: SyntaxKindId) -> Self {
        CompletedMarker { position, syntax_kind }
    }

    // TODO precede
}