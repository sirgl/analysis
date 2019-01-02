#![feature(test)]

extern crate test;
extern crate java_syntax;

use std::env;
use std::fs::File;
use std::io::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use java_syntax::*;
    use java_syntax::lexer::types::JavaTokenType;
    use logos::Logos;

    #[bench]
    fn bench_big_file(b: &mut Bencher) {
        let text = read_file();
        b.bytes = text.len() as u64;
        b.iter(|| {
            let mut lexer = JavaTokenType::lexer(text.as_str());
            let iterator = iter_for_lexer(lexer);
            let tokens: Vec<TokenInfo> = iterator.collect();
            tokens
        });
    }

    fn read_file() -> String {
        let mut f = File::open("./benches/Main.java").unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();
        contents
    }
}