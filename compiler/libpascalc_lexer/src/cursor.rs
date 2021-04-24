use std::str::Chars;

pub(in crate) const EOF_CHAR: char = '\0';

pub(in crate) struct Cursor<'a> {
    initial_len: usize,
    chars: Chars<'a>,
    #[cfg(debug_assertions)]
    prev: char
}

impl<'a> Cursor<'a> {
    pub(in crate) fn new(input: &'a str) -> Self {
        Self {
            initial_len: input.len(),
            chars: input.chars(),
            #[cfg(debug_assertions)]
            prev: EOF_CHAR
        }
    }

    pub(in crate) fn bump(&mut self) -> Option<char> {
        let r#char = self.chars.next()?;

        #[cfg(debug_assertions)]
        {
            self.prev = r#char;
        }

        Some(r#char)
    }

    pub(in crate) fn first(&self) -> char {
        self.nth_char(0)
    }

    pub(in crate) fn is_eof(&self) -> bool {
        self.chars().as_str().is_empty()
    }

    pub(in crate) fn len_consumed(&self) -> usize {
        self.initial_len - self.chars.as_str().len()
    }

    pub(in crate) fn prev(&self) -> char {
        #[cfg(debug_assertions)]
        {
            self.prev
        }

        #[cfg(not(debug_assertions))]
        {
            EOF_CHAR
        }
    }

    pub(in crate) fn second(&self) -> char {
        self.nth_char(1)
    }

    fn chars(&self) -> Chars<'a> {
        self.chars.clone()
    }

    fn nth_char(&self, n: usize) -> char {
        self.chars().nth(n).unwrap_or(EOF_CHAR)
    }
}
