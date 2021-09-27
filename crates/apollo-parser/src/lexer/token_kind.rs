/// Tokens generated by the lexer.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum TokenKind {
    Whitespace, // \r | \n |   | \t
    Comment,    // # comment
    Bang,       // !
    Dollar,     // $
    Amp,        // &
    Spread,     // ...
    Comma,      // ,
    Colon,      // :
    Eq,         // =
    At,         // @
    LParen,     // (
    RParen,     // )
    LBracket,   // [
    RBracket,   // ]
    LCurly,     // {
    RCurly,     // }
    Pipe,       // |
    Eof,

    // composite nodes
    Name,
    StringValue,
    Int,
    Float,

    // Root node
    Root,
}

// TODO: remove me
impl From<TokenKind> for rowan::SyntaxKind {
    fn from(kind: TokenKind) -> Self {
        Self(kind as u16)
    }
}

#[macro_export]
macro_rules! T {
    [!] => { $ crate :: TokenKind :: Bang } ;
    [$] => { $ crate :: TokenKind :: Dollar } ;
    [&] => { $ crate :: TokenKind :: Amp } ;
    [...] => { $ crate :: TokenKind :: Spread } ;
    [,] => { $ crate :: TokenKind :: Comma } ;
    [:] => { $ crate :: TokenKind :: Colon } ;
    [=] => { $ crate :: TokenKind :: Eq } ;
    [@] => { $ crate :: TokenKind :: At } ;
    ['('] => { $ crate :: TokenKind :: LParen } ;
    [')'] => { $ crate :: TokenKind :: RParen } ;
    ['['] => { $ crate :: TokenKind :: LBracket } ;
    [']'] => { $ crate :: TokenKind :: RBracket } ;
    ['{'] => { $ crate :: TokenKind :: LCurly } ;
    ['}'] => { $ crate :: TokenKind :: RCurly } ;
    [|] => { $ crate :: TokenKind :: Pipe } ;

    // composite nodes
    [name] => { $ crate :: TokenKind :: Name } ;
    [string] => { $ crate :: TokenKind :: StringValue} ;
    [int] => { $ crate :: TokenKind :: Int} ;
    [float] => { $ crate :: TokenKind :: Float} ;
}
