const TAG_LEN: u16 = 0b1000_0000_0000_0000;
const MAX_LEN: u32 = 0b0111_1111_1111_1111;

pub const DUMMY_SPAN: Span = Span { base_index: 0, len_tag: 0 };

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Span {
    base_index: u32,
    len_tag: u16
}
