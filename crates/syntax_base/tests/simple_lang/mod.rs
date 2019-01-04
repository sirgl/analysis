extern crate test;

mod lexer;
mod parser;
mod syntax;

use syntax_base::syntax_kind::*;
use syntax_base::syntax::SyntaxDefinition;
use syntax_base::lexer::Lexer;
use syntax_base::parser::Parser;
use syntax_base::syntax::SyntaxInfo;
use syntax_base::tokens::TokenInfo;
use crate::simple_lang::lexer::SimpleLexer;
use crate::simple_lang::syntax::infos;
use crate::simple_lang::parser::parse_file;
use syntax_base::parser::parser_api::ParserApi;
use errors::TextDiagnostic;
use syntax_base::parser::parser_impl::sink::GreenTreeEventSink;
use rowan::GreenNode;
use rowan::Types;
use rowan::SyntaxNode;
use syntax_base::nodes::PlatformTypes;


pub struct SimpleLangSyntax {}

impl SyntaxDefinition for SimpleLangSyntax {
    fn lexer(&self) -> Option<Box<Lexer>> {
        Some(Box::new(SimpleLexer {}))
    }

    fn parser<'a>(&self) -> Option<Box<Parser>> {
        Some(Box::new(SimpleParser {}))
    }


    fn _syntax_kind_info(&self, id: SyntaxKindId) -> &SyntaxInfo {
        match id {
            SyntaxKindId::TOMBSTONE => &SyntaxInfo { name: "TOMBSTONE" },
            SyntaxKindId::END => &SyntaxInfo { name: "END" },
            SyntaxKindId::ERROR => &SyntaxInfo { name: "ERROR" },
            SyntaxKindId::ROOT => &SyntaxInfo { name: "ROOT" },
            SyntaxKindId::INJECTED_ROOT => &SyntaxInfo { name: "INJECTED_ROOT" },
            _ => infos(id)
        }
    }

    fn id_by_syntax_kind_name(&self, name: &str) -> Option<SyntaxKindId> {
        unimplemented!()
    }
}


struct Tree {}

struct SimpleParser {}

impl Parser for SimpleParser {
    fn parse(&self, text: &str, tokens: Vec<TokenInfo>) -> GreenNode<PlatformTypes> {
        let mut api = ParserApi::new(tokens, text);
        let file: GreenNode<PlatformTypes> = parse_file(api, GreenTreeEventSink::new());
//        let node = SyntaxNode::new(file, Vec::new());
        file
    }
}