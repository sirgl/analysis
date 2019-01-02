use text_unit::TextUnit;
use text_unit::TextRange;

pub struct ParseError {
    pub location: Location,
    pub error_kind: ParseErrorKind,
}

pub enum Location {
    Offset(TextUnit),
    Range(TextRange),
}

pub enum ParseErrorKind {

}