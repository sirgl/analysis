// TODO Is it really highlight type?
//  probably not, but what is it?
pub enum HighlightType {
    CompilationError,
    DefinitelyError,
    BadPractice,
    ZeroTolerance,
    // ...
}

pub enum Severity {
    Error,
    Warning
}

pub trait Diagnostic {
    fn text(&self) -> String;
}

pub struct TextDiagnostic<'a> {
    text: &'a str
}

impl Diagnostic for TextDiagnostic {
    fn text(&self) -> String {
        self.
    }
}