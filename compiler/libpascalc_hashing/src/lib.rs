//! `libpascalc_hashing`
//!
//! Custom hash algorithm implemented for the RustPascal compiler.

use std::{
    collections::{
        HashMap,
        HashSet
    },
    hash::{
        BuildHasherDefault,
        Hasher
    }
};

#[cfg(target_pointer_width = "32")]
const K: usize = 0x9e3779b9;
#[cfg(target_pointer_width = "64")]
const K: usize = 0x517cc1b727220a95;

pub struct PascalcHasher {
    hash: usize
}

impl Default for PascalcHasher {
    #[inline]
    fn default() -> Self {
        Self { hash: 0 }
    }
}
