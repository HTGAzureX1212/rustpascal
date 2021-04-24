use expect_test::{
    expect,
    Expect
};

use crate::lexical_analyze;

#[test]
fn hello_world_test() {
    let source = include_str!("../../tests/lexer/lexer_helloworld_test.pas");

    let expect = expect![[r"
    Lexeme { type: Identifier, len: 7 }
    Lexeme { type: Whitespace, len: 1 }
    Lexeme { type: Identifier, len: 10 }
    Lexeme { type: Semicolon, len: 1 }
    Lexeme { type: Whitespace, len: 2 }
    Lexeme { type: Identifier, len: 5 }
    Lexeme { type: Whitespace, len: 6 }
    Lexeme { type: Identifier, len: 7 }
    Lexeme { type: Whitespace, len: 1 }
    Lexeme { type: LeftRound, len: 1 }
    Lexeme { type: Literal { type: String { terminated: true } }, len: 14 }
    Lexeme { type: RightRound, len: 1 }
    Lexeme { type: Semicolon, len: 1 }
    Lexeme { type: Whitespace, len: 2 }
    Lexeme { type: Identifier, len: 3 }
    Lexeme { type: Dot, len: 1 }
    "]];
    let actual = lexical_analyze(source).map(|lexeme| {
        format!("{:?}\n", lexeme)
    }).collect::<String>();

    expect.assert_eq(&actual);
}
