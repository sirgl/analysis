use crate::simple_lang::*;
use syntax_base::syntax_kind::SyntaxKindId;
use syntax_base::parser::parser_api::ParserApi;

fn parse_file(p: &mut ParserApi) {
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
