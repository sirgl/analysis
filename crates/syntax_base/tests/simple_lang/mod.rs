extern crate test;

mod lexer;
mod parser;
mod syntax;
mod descriptions;

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
use syntax_base::nodes::syntax::PlatformRootData;
use syntax_base::nodes::syntax::SyntaxNode;
use syntax_base::nodes::syntax::PlatformTypes;
use syntax_base::nodes::syntax::GreenNode;


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


struct SimpleParser {}

impl Parser for SimpleParser {
    fn parse(&self, text: &str, tokens: Vec<TokenInfo>) -> SyntaxNode {
        let mut api = ParserApi::new(tokens, text);
        let file: GreenNode = parse_file(api, GreenTreeEventSink::new());
        SyntaxNode(rowan::SyntaxNode::new(file,  PlatformRootData::new() ))
    }
}