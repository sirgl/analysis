use syntax_base::syntax_kind::SyntaxKindId;
use syntax_base::parser::parser_api::ParserApi;
use crate::simple_lang::syntax::*;
use syntax_base::parser::parser_impl::sink::ParseEventSink;
use syntax_base::syntax_kind_set::SmallSyntaxKindSet;

pub fn parse_file<T, S: ParseEventSink<T>>(mut p: ParserApi, sink: S) -> T {
    parse_file_internal(&mut p);
    p.build(sink)
}

fn parse_file_internal(p: &mut ParserApi) {
    let mut file = p.start();
    while !p.at(SyntaxKindId::END) {
        eprintln!("Before fun");
        if !p.at(FUN_KW) {
            eprintln!("Not at fun");
            p.err_recover("function expected", FUN_KW);
            break
        }
        eprintln!("parse fun");
        parse_fun(p);
    }
    file.complete(p, SyntaxKindId::ROOT);
}

fn parse_fun(p: &mut ParserApi) {
    assert!(p.at(FUN_KW));
    let function = p.start();
    p.bump();
    if !p.at(ID) {

        p.err_recover("identifier expected", LPAR)
    }
    if !p.at(LPAR) {
        p.err_recover("", RPAR)
    }
    parse_args(p);
    function.complete(p, FUNCTION);
    // TODO body
}

fn parse_args(p: &mut ParserApi) {
    assert!(p.at(LPAR));
    let args = p.start();
    p.leaf(LPAR);
    while !p.at(RPAR) {
        p.leaf(ID);
        if p.at(COMMA) {
            p.leaf(COMMA);
        }
    }
    p.leaf(RPAR);
    args.complete(p, ARGS);
}

#[cfg(test)]
mod tests {
    use crate::simple_lang::SimpleLangSyntax;
    use syntax_base::syntax::SyntaxDefinition;
    use std::path::PathBuf;
    use crate::test_support::ParserTest;

    #[test]
    fn test1() {
        by_file("1")
    }

    const EXT: &str = "simple";
    const BASE_PATH: &str = "./tests/data/simple/parser";

    fn by_file(name: &str) {
        let syntax = SimpleLangSyntax {};
        let lexer = syntax.lexer().unwrap();
        let parser = syntax.parser().unwrap();
        let test = ParserTest::new(
            &syntax,
            lexer.as_ref(),
            parser.as_ref(),
            EXT.to_string(),
            PathBuf::from(BASE_PATH)
        );
        test.test(name);
    }
}
