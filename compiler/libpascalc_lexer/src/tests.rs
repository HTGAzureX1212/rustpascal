use expect_test::expect;

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

#[test]
fn comments_test() {
    let source = include_str!("../../tests/lexer/lexer_comment_test.pas");

    let expect = expect![[r"
    Lexeme { type: LineComment, len: 16 }
    Lexeme { type: Whitespace, len: 1 }
    Lexeme { type: BlockComment, len: 31 }
    Lexeme { type: Whitespace, len: 2 }
    Lexeme { type: BlockComment, len: 23 }
    Lexeme { type: Whitespace, len: 1 }
    Lexeme { type: BlockComment, len: 46 }
    Lexeme { type: Whitespace, len: 1 }
    "]];
    let actual = lexical_analyze(source).map(|lexeme| {
        format!("{:?}\n", lexeme)
    }).collect::<String>();

    expect.assert_eq(&actual);
}

#[test]
fn integers_test() {
    let source = include_str!("../../tests/lexer/lexer_integers_test.pas");

    let expect = expect![[r"
    "]];
    let actual = lexical_analyze(source).map(|lexeme| {
        format!("{:?}\n", lexeme)
    }).collect::<String>();

    expect.assert_eq(&actual);
}
