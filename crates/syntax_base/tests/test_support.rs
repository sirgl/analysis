use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use syntax_base::lexer::Lexer;
use syntax_base::parser::Parser;
use syntax_base::syntax::SyntaxDefinition;
use syntax_base::tokens::convert_to_fixed;
use syntax_base::tokens::FixedToken;

/// rewrite content of test file with actual data
const REWRITE: bool = false;

/// Reads expected and actual files and feeds them to assert function
/// Assert function: (source file data) -> actual_data
pub fn test_by_file<F: Fn(&str) -> String>(file: PathBuf, expected_data: PathBuf, assert_func: F) {
    let mut source_file = File::open(file).expect(format!("Source data not found").as_str());
    let mut expected_file = match File::open(expected_data.clone()) {
        Ok(expected_file) => expected_file,
        Err(_) => File::create(expected_data.clone()).unwrap(),
    };

    let mut source_text = String::new();
    let mut expected_data_text = String::new();
    source_file.read_to_string(&mut source_text).unwrap();
    expected_file.read_to_string(&mut expected_data_text).unwrap();
    let actual_text = assert_func(source_text.as_str());
    if REWRITE {
        fs::write(expected_data, actual_text).unwrap();
    } else {
        assert_eq_text(expected_data_text.as_str(), actual_text.as_str());
    }
}

/// Testing relative
pub fn test_by_file_rel<F: Fn(&str) -> String>(
    base_path: &Path,
    relative_source_name: String,
    relative_expected_data_name: String,
    assert_func: F,
) {
    let source_path = base_path.join(PathBuf::from(relative_source_name));
    let expected_path = base_path.join(PathBuf::from(relative_expected_data_name));
    test_by_file(source_path, expected_path, assert_func);
}

pub fn test_by_file_rel_ext<F: Fn(&str) -> String>(
    base_path: &Path,
    extension: String,
    file_name: String,
    assert_func: F,
) {
    let mut file_name = file_name.to_string();
    file_name.push_str(".");
    file_name.push_str(extension.as_str());
    let mut expected_name = file_name.clone();
    expected_name.push_str(".txt");
    test_by_file_rel(base_path, file_name, expected_name, assert_func)
}

pub struct LexerTest<'a> {
    syntax_def: &'a SyntaxDefinition,
    lexer: &'a Lexer,
    extension: String,
    base_path: PathBuf,
}

impl<'a> LexerTest<'a> {
    pub fn new(syntax_def: &'a SyntaxDefinition, lexer: &'a Lexer, extension: String, base_path: PathBuf) -> Self {
        LexerTest { syntax_def, lexer, extension, base_path }
    }
}


impl<'a> LexerTest<'a> {
//    pub fn by_text(&self, text: &str, expected: &str) {}

    /// do test, based on some file
    /// file name without extension
    pub fn by_file(&self, file_name: &str) {
        test_by_file_rel_ext(
            self.base_path.as_path(),
            self.extension.clone(),
            file_name.to_string(),
            |program| {
                let lexer = self.lexer;
                let tokens = lexer.tokenize(program);
                convert_to_fixed(program, tokens, self.syntax_def)
                    .iter()
                    .map(|token: &FixedToken| format!("{}", token))
                    .collect::<Vec<String>>()
                    .join("\n")
            });
    }
}

pub fn assert_eq_text(expected: &str, actual: &str) {
    if expected != actual {
        eprintln!("actual:\n{}\n--------\n", actual);
        assert_eq!(expected, actual)
    }
}

pub struct ParserTest<'a> {
    syntax_def: &'a SyntaxDefinition,
    lexer: &'a Lexer,
    parser: &'a Parser,
    extension: String,
    base_path: PathBuf,
}

impl<'a> ParserTest<'a> {
    fn test(&self, file_name: &str) {
        test_by_file_rel_ext(
            self.base_path.as_path(),
            self.extension.clone(),
            file_name.to_string(),
            |program| {
                let lexer = self.lexer;
                let tokens = lexer.tokenize(program);
                let node = self.parser.parse(program, tokens);
//                let id = node.get_type_id();
//                print!("{?:}", id);
                "".to_string() // TODO
            });
    }
}

//pub fn dump_tree(node: SyntaxNodeRef) -> String {
//    let buffer = String::new();
//    buffer.

//}
