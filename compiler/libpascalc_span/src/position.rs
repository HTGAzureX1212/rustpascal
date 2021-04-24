#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BytePosition(pub u32);

impl Position for BytePosition {
    fn from_usize(value: usize) -> Self {
        Self(value as u32)
    }

    fn to_usize(&self) -> usize {
        self.0 as usize
    }

    fn from_u32(value: u32) -> Self {
        Self(value)
    }

    fn to_u32(&self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CharPosition(pub usize);

impl Position for CharPosition {
    fn from_usize(value: usize) -> Self {
        Self(value)
    }

    fn to_usize(&self) -> usize {
        self.0
    }

    fn from_u32(value: u32) -> Self {
        Self(value as usize)
    }

    fn to_u32(&self) -> u32 {
        self.0 as u32
    }
}

pub trait Position {
    fn from_usize(value: usize) -> Self;

    fn to_usize(&self) -> usize;

    fn from_u32(value: u32) -> Self;

    fn to_u32(&self) -> u32;
}
