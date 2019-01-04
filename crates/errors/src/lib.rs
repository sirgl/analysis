use text_unit::TextUnit;
use text_unit::TextRange;

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
    fn location(&self) -> Location;
}

#[derive(Debug)]
pub struct TextDiagnostic {
    text: String,
    location: Location
}

impl TextDiagnostic {
    pub fn new(text: String, location: Location) -> Self {
        TextDiagnostic { text, location }
    }
}

impl Diagnostic for TextDiagnostic {
    fn text(&self) -> String {
        self.text.clone()
    }

    fn location(&self) -> Location {
        self.location
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Location {
    Offset(TextUnit),
    Range(TextRange),
}