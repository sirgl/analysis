use crate::syntax_kind::SyntaxKindId;
use text_unit::TextRange;
use text_unit::TextUnit;
use core::fmt;
use std::fmt::Formatter;
use std::fmt::Error;
use smol_str::SmolStr;
use crate::syntax::SyntaxDefinition;
use crate::escape_str;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct TokenInfo {
    pub token_type: SyntaxKindId,
    pub len: TextUnit
}

pub fn convert_to_fixed<'a>(text: &'a str, tokens: Vec<TokenInfo>, def: &'a SyntaxDefinition) -> Vec<FixedToken<'a>> {
    let mut offset = TextUnit::from(0);
    tokens.iter().map(|t| {
        let range = TextRange::from_to(offset, offset + t.len);
        offset += t.len;
        let token = FixedToken::new(t.token_type, range,  SmolStr::new(&text[range]), def);
        token
    }).collect()
}


/// Fat fixed representation of token used to display it
pub struct FixedToken<'a> {
    pub token_type: SyntaxKindId,
    pub range: TextRange,
    pub text: SmolStr,
    pub def: &'a SyntaxDefinition
}

impl<'a> FixedToken<'a> {
    pub fn new(token_type: SyntaxKindId, range: TextRange, text: SmolStr, def: &'a SyntaxDefinition) -> Self {
        FixedToken { token_type, range, text, def }
    }
}

impl <'a>fmt::Display for FixedToken<'a> {
    fn fmt<'b>(&self, f: &mut Formatter<'b>) -> Result<(), Error> {
        // TODO token_type
        let name = self.def._syntax_kind_info(self.token_type).name;
        write!(f, "{}{}@{}", name, self.range, escape_str(self.text.as_str()))
    }
}
