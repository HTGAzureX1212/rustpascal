#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum Base {
    Base2,

    Base8,

    Base10,

    Base16
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum LexemeType {
    LineComment,
    BlockComment,

    Whitespace,
    Identifier,
    RawIdentifier,
    Literal { r#type: LiteralType },

    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    LeftAngle,
    RightAngle,
    LeftSquare,
    RightSquare,
    Dot,
    Comma,
    LeftRound,
    RightRound,
    Colon,
    Caret,
    At,
    Pound,
    Semicolon,

    Unknown
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum LiteralType {
    Integer {
        base: Base,
        empty: bool
    },

    // Float {
    //     base: Base,
    //     empty_expo: bool,
    // },

    String {
        terminated: bool
    }
}
