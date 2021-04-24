//! `libpascalc_span`
//!
//! Source positions as well as some helper functions implemented for the RustPascal compiler.

pub mod position;

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SpanData {
    pub low: position::BytePosition,
    pub high: position::BytePosition
}

impl SpanData {
    
}
