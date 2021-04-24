//! `libpascalc_lexer`
//!
//! An implementation of a lexical analysis for the RustPascal compiler.

#![feature(control_flow_enum)]

use std::iter;

mod cursor;
pub mod ext;
pub mod lexeme;
#[cfg(test)]
mod tests;

use crate::{
    cursor::EOF_CHAR,
    ext::CharExt,
    lexeme::r#type::{
        Base,
        LexemeType,
        LiteralType
    }
};

impl<'a> cursor::Cursor<'a> {
    // Advancing
    fn next_lexeme(&mut self) -> Lexeme {
        let character = self.bump().unwrap();
        let r#type = match character {
            '/' => {
                match self.first() {
                    '/' => self.line_comment(),
                    _ => LexemeType::Slash
                }
            }
            r#char if CharExt::is_whitespace(r#char) => {
                self.whitespace()
            }
            '&' => {
                match self.first() {
                    r#char if r#char.is_id_start() => {
                        self.identifier()
                    }
                    r#char if r#char.is_digit(8) => {
                        let r#type = self.number();

                        LexemeType::Literal { r#type }
                    }
                    _ => LexemeType::Unknown
                }
            }
            '$' => {
                match self.first() {
                    r#char if r#char.is_digit(16) => {
                        let r#type = self.number();

                        LexemeType::Literal { r#type }
                    }
                    _ => LexemeType::Unknown
                }
            }
            '%' => {
                match self.first() {
                    r#char if r#char.is_digit(2) => {
                        let r#type = self.number();

                        LexemeType::Literal { r#type }
                    }
                    _ => LexemeType::Unknown
                }
            }
            r#char if r#char.is_digit(10) => {
                let r#type = self.number();

                LexemeType::Literal { r#type }
            }
            r#char if r#char.is_id_start() => {
                self.identifier()
            }
            '+' => {
                LexemeType::Plus
            }
            '-' => {
                LexemeType::Minus
            }
            '*' => {
                LexemeType::Star
            }
            '=' => {
                LexemeType::Equal
            }
            '<' => {
                LexemeType::LeftAngle
            }
            '>' => {
                LexemeType::RightAngle
            }
            '[' => {
                LexemeType::LeftSquare
            }
            ']' => {
                LexemeType::RightSquare
            }
            '.' => {
                LexemeType::Dot
            }
            ',' => {
                LexemeType::Comma
            }
            '(' => {
                if self.first() == '*' {
                    self.block_comment()
                }
                else {
                    LexemeType::LeftRound
                }
            }
            ')' => {
                LexemeType::RightRound
            }
            ':' => {
                LexemeType::Colon
            }
            '^' => {
                LexemeType::Caret
            }
            '@' => {
                LexemeType::At
            }
            '#' => {
                LexemeType::Pound
            }
            ';' => {
                LexemeType::Semicolon
            }
            '\'' => {
                let terminated = self.character_string();

                LexemeType::Literal { r#type: LiteralType::String { terminated } }
            }
            _ => LexemeType::Unknown
        };

        Lexeme::new(r#type, self.len_consumed())
    }

    // "Eating"

    fn eat_decimal_digits(&mut self) -> bool {
        let mut has_digits = false;

        loop {
            match self.first() {
                '_' => {
                    self.bump();
                }
                '0'..='9' => {
                    has_digits = true;
                    self.bump();
                }
                _ => break
            }
        };

        has_digits
    }

    fn eat_float_exponent(&mut self) -> bool {
        debug_assert!(self.prev() == 'e' || self.prev() == 'E');

        if ['-', '+'].contains(&self.first()) {
            self.bump();
        }

        self.eat_decimal_digits()
    }

    fn eat_hexadecimal_digits(&mut self) -> bool {
        let mut has_digits = false;

        loop {
            match self.first() {
                '_' => {
                    self.bump();
                }
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    has_digits = true;
                    self.bump();
                }
                _ => break
            }
        };

        has_digits
    }

    fn eat_identifier(&mut self) {
        if !self.first().is_id_start() {
            return;
        }

        self.bump();
        self.eat_while(char::is_id_continue)
    }

    fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.first()) && !self.is_eof() {
            self.bump();
        }
    }

    // "Actual Analysis Through Source"
    fn block_comment(&mut self) -> LexemeType {
        match self.prev() {
            '(' if self.first() == '*' => {
                self.bump();

                while let Some(character) = self.bump() {
                    match character {
                        '*' if self.first() == ')' => {
                            self.bump();

                            break;
                        },
                        _ => ()
                    }
                }
            }
            '{' => {
                while let Some(character) = self.bump() {
                    match character {
                        '}' => {
                            self.bump();

                            break;
                        },
                        _ => ()
                    }
                }
            }
            _ => unreachable!("This code should be unreachable as if there are only two variants of block comments in this dialect.")
        };

        LexemeType::BlockComment
    }

    fn character_string(&mut self) -> bool {
        debug_assert!(self.prev() == '\'');

        loop {
            match self.first() {
                '\'' => {
                    self.bump();
                    return true;
                }
                '\n' => break,
                EOF_CHAR if self.is_eof() => break,
                _ => {
                    self.bump();
                }
            }
        }

        false
    }

    fn line_comment(&mut self) -> LexemeType {
        debug_assert!(self.prev() == '/' && self.first() == '/');

        self.bump();
        self.eat_while(|character| character != '\n');
        LexemeType::LineComment
    }

    fn identifier(&mut self) -> LexemeType {
        debug_assert!(self.prev().is_id_start());

        self.eat_while(char::is_id_continue);
        LexemeType::Identifier
    }

    fn number(&mut self) -> LiteralType {
        let mut base = Base::Base10;

        match self.prev() {
            '$' => {
                base = Base::Base16;
                self.eat_hexadecimal_digits();
            }
            '&' => {
                base = Base::Base8;
                self.eat_decimal_digits();
            }
            '%' => {
                base = Base::Base2;
                self.eat_decimal_digits();
            }
            _ => {
                self.eat_decimal_digits();
            }
        }

        LiteralType::Integer { base, empty: false }
    }

    fn raw_identifier(&mut self) -> LexemeType {
        debug_assert!(self.prev() == '&' && self.first().is_id_start());

        self.eat_identifier();
        LexemeType::RawIdentifier
    }

    fn whitespace(&mut self) -> LexemeType {
        debug_assert!(CharExt::is_whitespace(self.prev()));

        self.eat_while(CharExt::is_whitespace);
        LexemeType::Whitespace
    }
}

#[derive(Debug)]
pub struct Lexeme {
    pub r#type: LexemeType,
    pub len: usize
}

impl Lexeme {
    pub fn new(r#type: LexemeType, len: usize) -> Self {
        Self {
            r#type,
            len
        }
    }
}

pub fn first_lexeme(input: &str) -> Lexeme {
    debug_assert!(!input.is_empty());

    cursor::Cursor::new(input).next_lexeme()
}

pub fn lexical_analyze(mut input: &str) -> impl Iterator<Item = Lexeme> + '_ {
    iter::from_fn(move || {
        if input.is_empty() {
            return None;
        }

        let lexeme = first_lexeme(input);
        input = &input[lexeme.len..];

        Some(lexeme)
    })
}
