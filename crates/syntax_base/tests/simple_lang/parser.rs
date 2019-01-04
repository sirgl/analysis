use syntax_base::syntax_kind::SyntaxKindId;
use syntax_base::parser::parser_api::ParserApi;
use crate::simple_lang::syntax::*;
use syntax_base::parser::parser_impl::sink::ParseEventSink;

pub fn parse_file<T, S: ParseEventSink<T>>(mut p: ParserApi, sink: S) -> T {
    parse_file_internal(&mut p);
    p.build(sink)
}

fn parse_file_internal(p: &mut ParserApi) {
    let mut file = p.start();
    while p.at(SyntaxKindId::END) {
        if !p.at(FUN_KW) {
            p.error("function expected");
        }
        parse_fun(p);
    }
    file.complete(p, SyntaxKindId::ROOT);
}

fn parse_fun(p: &mut ParserApi) {
    assert!(p.at(FUN_KW));
    p.bump();
    if !p.at(ID) {
        p.err_recover("identifier expected", LPAR)
    }
    if !p.at(LPAR) {
        p.err_recover("", RPAR)
    }
    parse_args(p);
    // TODO body
}

fn parse_args(p: &mut ParserApi) {
    assert!(p.at(LPAR));
    let mut args = p.start();
    while !p.at(RPAR) {
        p.leaf(ID);
        if p.at(COMMA) {
            p.bump();
        }
    }
    args.complete(p, ARGS);
}
