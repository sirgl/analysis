use std::fmt::Debug;

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Eq)]
pub enum JavaTokenType {
    #[end]
    End,

    #[error]
    Error,

    #[regex = "[\r|\n|\t| ]" ]
    Whitespace,
    // TODO
    #[regex = "([_|$|a-z|A-Z]([_|$|a-z|A-Z|_|$0-9])*)" ]
    Identifier,
    #[token = "abstract" ]
    AbstractKw,
    #[token = "continue" ]
    ContinueKw,
    #[token = "for" ]
    ForKw,
    #[token = "new" ]
    NewKw,
    #[token = "switch" ]
    SwitchKw,
    #[token = "assert" ]
    AssertKw,
    #[token = "default" ]
    DefaultKw,
    #[token = "if" ]
    IfKw,
    #[token = "package" ]
    PackageKw,
    #[token = "synchronized" ]
    SynchronizedKw,
    #[token = "boolean" ]
    BooleanKw,
    #[token = "do" ]
    DoKw,
    #[token = "goto" ]
    GotoKw,
    #[token = "private" ]
    PrivateKw,
    #[token = "this" ]
    ThisKw,
    #[token = "break" ]
    BreakKw,
    #[token = "double" ]
    DoubleKw,
    #[token = "implements" ]
    ImplementsKw,
    #[token = "protected" ]
    ProtectedKw,
    #[token = "throw" ]
    ThrowKw,
    #[token = "byte" ]
    ByteKw,
    #[token = "else" ]
    ElseKw,
    #[token = "import" ]
    ImportKw,
    #[token = "public" ]
    PublicKw,
    #[token = "throws" ]
    ThrowsKw,
    #[token = "case" ]
    CaseKw,
    #[token = "enum" ]
    EnumKw,
    #[token = "instanceof" ]
    InstanceofKw,
    #[token = "return" ]
    ReturnKw,
    #[token = "transient" ]
    TransientKw,
    #[token = "catch" ]
    CatchKw,
    #[token = "extends" ]
    ExtendsKw,
    #[token = "int" ]
    IntKw,
    #[token = "short" ]
    ShortKw,
    #[token = "try" ]
    TryKw,
    #[token = "char" ]
    CharKw,
    #[token = "final" ]
    FinalKw,
    #[token = "interface" ]
    InterfaceKw,
    #[token = "static" ]
    StaticKw,
    #[token = "void" ]
    VoidKw,
    #[token = "class" ]
    ClassKw,
    #[token = "finally" ]
    FinallyKw,
    #[token = "long" ]
    LongKw,
    #[token = "strictfp" ]
    StrictfpKw,
    #[token = "volatile" ]
    VolatileKw,
    #[token = "const" ]
    ConstKw,
    #[token = "float" ]
    FloatKw,
    #[token = "native" ]
    NativeKw,
    #[token = "super" ]
    SuperKw,
    #[token = "while" ]
    WhileKw,
    // TODO
//    #[regex = "[0-9]+" ]
    #[regex = "[0-9][_0-9]*[lL]?" ]
    IntegerLiteral,
    #[token = "=" ]
    Eq,
    #[token = ">" ]
    Gt,
    #[token = "<" ]
    Lt,
    #[token = "!" ]
    Excl,
    #[token = "~" ]
    Tilde,
    #[token = "?" ]
    Question,
    #[token = ":" ]
    Colon,
    #[token = "->" ]
    DashGt,
    #[token = "==" ]
    EqEq,
    #[token = ">=" ]
    GtEq,
    #[token = "<=" ]
    LtEq,
    #[token = "!=" ]
    ExclEq,
    #[token = "&&" ]
    AndAnd,
    #[token = "||" ]
    OrOr,
    #[token = "++" ]
    PlusPlus,
    #[token = "--" ]
    DashDash,
    #[token = "+" ]
    Plus,
    #[token = "-" ]
    Dash,
    #[token = "*" ]
    Asterisk,
    #[token = "/" ]
    Div,
    #[token = "&" ]
    And,
    #[token = "|" ]
    Or,
    #[token = "^" ]
    Caret,
    #[token = "%" ]
    Percent,
    #[token = "<<" ]
    LtLt,
    #[token = ">>" ]
    GtGt,
    #[token = ">>>" ]
    GtGtGt,
    #[token = "+=" ]
    PlusEq,
    #[token = "-=" ]
    DashEq,
    #[token = "*=" ]
    AsteriskEq,
    #[token = "/=" ]
    DivEq,
    #[token = "&=" ]
    AndEq,
    #[token = "|=" ]
    OrEq,
    #[token = "^=" ]
    CaretEq,
    #[token = "%=" ]
    PercentEq,
    #[token = "<<=" ]
    LtLtEq,
    #[token = ">>=" ]
    GtGtEq,
    #[token = ">>>=" ]
    GtGtGtEq,
    #[token = "(" ]
    Lpar,
    #[token = ")" ]
    Rpar,
    #[token = "{" ]
    LBrace,
    #[token = "}" ]
    RBrace,
    #[token = "[" ]
    LBracket,
    #[token = "]" ]
    RBracket,
    #[token = ";" ]
    Semicolon,
    #[token = "," ]
    Comma,
    #[token = "." ]
    Dot,
    #[token = "..." ]
    DotDotDot,
    #[token = "@" ]
    At,
    #[token = "::" ]
    ColonColon
}