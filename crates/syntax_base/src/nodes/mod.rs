use errors::TextDiagnostic;
use rowan::TreeRoot;
use crate::language::LanguageId;
use crate::syntax_kind_set::IterableSyntaxKindSet;
use crate::syntax_kind::SyntaxKind;
use smol_str::SmolStr;
use rowan::WalkEvent;

pub mod algo;
pub mod syntax;
pub mod syntax_text;
