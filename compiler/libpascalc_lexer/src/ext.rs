pub trait CharExt {
    fn is_whitespace(self) -> bool;

    fn is_id_start(self) -> bool;

    fn is_id_continue(self) -> bool;
}

impl CharExt for char {
    fn is_whitespace(self) -> bool {
        matches!(
            self,

            '\u{0009}'
            | '\u{000A}'
            | '\u{000B}'
            | '\u{000C}'
            | '\u{000D}'
            | '\u{0020}'

            | '\u{0085}'

            | '\u{200E}'
            | '\u{200F}'

            | '\u{2028}'
            | '\u{2029}'
        )
    }

    fn is_id_start(self) -> bool {
        ('a'..='z').contains(&self)
            || ('A'..='Z').contains(&self)
            || self == '_'
    }

    fn is_id_continue(self) -> bool {
        ('a'..='z').contains(&self)
            || ('A'..='Z').contains(&self)
            || ('0'..='9').contains(&self)
            || self == '_'
    }
}

pub trait StrExt {
    fn is_lexically_ident(&self) -> bool;
}

impl StrExt for str {
    fn is_lexically_ident(&self) -> bool {
        let mut chars = self.chars();

        if let Some(start) = chars.next() {
            return start.is_id_start() && chars.all(char::is_id_continue);
        }

        false
    }
}
