use crate::syntax::SyntaxDefinition;


// TODO is it really required to be in this module?

#[derive(Debug)]
pub struct Language {
    pub id: LanguageId,
    name: &'static str,
}

impl Language {
    pub fn new(id: LanguageId, name: &'static str) -> Self {
        Language { id, name }
    }

    fn name(&self) -> &'static str {
        self.name
    }
}

#[derive(Debug, Copy, Clone)]
pub struct LanguageId(u16);


pub trait LanguageCapabilities {
    fn syntax_definition() -> Option<Box<dyn SyntaxDefinition>>;
    // TODO formatter, inspections, code completion and so on
}