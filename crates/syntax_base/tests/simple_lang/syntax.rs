// This file is autogenerated, not supposed for manual editing.

use syntax_base::syntax_kind::SyntaxKindId;
use syntax_base::syntax::SyntaxInfo;

pub const LBRACE: SyntaxKindId = SyntaxKindId::new(5);
pub const RBRACE: SyntaxKindId = SyntaxKindId::new(6);
pub const LPAR: SyntaxKindId = SyntaxKindId::new(7);
pub const RPAR: SyntaxKindId = SyntaxKindId::new(8);
pub const COMMA: SyntaxKindId = SyntaxKindId::new(9);
pub const PLUS: SyntaxKindId = SyntaxKindId::new(10);
pub const MINUS: SyntaxKindId = SyntaxKindId::new(11);
pub const MUL: SyntaxKindId = SyntaxKindId::new(12);
pub const DIV: SyntaxKindId = SyntaxKindId::new(13);
pub const EQ: SyntaxKindId = SyntaxKindId::new(14);
pub const SEMI: SyntaxKindId = SyntaxKindId::new(15);
pub const FUN_KW: SyntaxKindId = SyntaxKindId::new(16);
pub const ID: SyntaxKindId = SyntaxKindId::new(17);
pub const NUM: SyntaxKindId = SyntaxKindId::new(18);
pub const WHITESPACE: SyntaxKindId = SyntaxKindId::new(19);
pub const ARGS: SyntaxKindId = SyntaxKindId::new(20);
pub const FUNCTION: SyntaxKindId = SyntaxKindId::new(21);


pub fn infos(id: SyntaxKindId) -> &'static SyntaxInfo  {
    match id {
        LBRACE => &SyntaxInfo { name: "LBRACE" },
        RBRACE => &SyntaxInfo { name: "RBRACE" },
        LPAR => &SyntaxInfo { name: "LPAR" },
        RPAR => &SyntaxInfo { name: "RPAR" },
        COMMA => &SyntaxInfo { name: "COMMA" },
        PLUS => &SyntaxInfo { name: "PLUS" },
        MINUS => &SyntaxInfo { name: "MINUS" },
        MUL => &SyntaxInfo { name: "MUL" },
        DIV => &SyntaxInfo { name: "DIV" },
        EQ => &SyntaxInfo { name: "EQ" },
        SEMI => &SyntaxInfo { name: "SEMI" },
        FUN_KW => &SyntaxInfo { name: "FUN_KW" },
        ID => &SyntaxInfo { name: "ID" },
        NUM => &SyntaxInfo { name: "NUM" },
        WHITESPACE => &SyntaxInfo { name: "WHITESPACE" },
        ARGS => &SyntaxInfo { name: "ARGS" },
        FUNCTION => &SyntaxInfo { name: "FUNCTION" },
        _ => panic!("Bad syntax id")
    }
}