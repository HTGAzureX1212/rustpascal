//! `libpascalc_hashing`
//!
//! Custom hash algorithm implemented for the RustPascal compiler.

use std::{
    collections::{
        HashMap,
        HashSet
    },
    convert::TryInto,
    hash::{
        BuildHasherDefault,
        Hasher
    },
    mem::size_of,
    ops::BitXor
};

#[cfg(target_pointer_width = "32")]
const K: usize = 0x9e3779b9;
#[cfg(target_pointer_width = "64")]
const K: usize = 0x517cc1b727220a95;

pub struct PascalcHasher {
    hash: usize
}

impl PascalcHasher {
    #[inline]
    fn add_to_hash(&mut self, value: usize) {
        self.hash = self.hash.rotate_left(5).bitxor(value).wrapping_mul(K);
    }
}

impl Default for PascalcHasher {
    #[inline]
    fn default() -> Self {
        Self { hash: 0 }
    }
}

impl Hasher for PascalcHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.hash as u64
    }

    #[inline]
    fn write(&mut self, mut bytes: &[u8]) {
        #[cfg(target_pointer_width = "32")]
            let read_usize = |bytes: &[u8]| u32::from_ne_bytes(bytes[..4].try_into().unwrap());
        #[cfg(target_pointer_width = "64")]
            let read_usize = |bytes: &[u8]| u64::from_ne_bytes(bytes[..8].try_into().unwrap());

        let mut hash = FxHasher { hash: self.hash };
        assert!(size_of::<usize>() <= 8);

        while bytes.len() >= size_of::<usize>() {
            hash.add_to_hash(read_usize(bytes) as usize);
            bytes = &bytes[size_of::<usize>()..];
        }

        if (size_of::<usize>() > 4) && (bytes.len() >= 4) {
            hash.add_to_hash(u32::from_ne_bytes(bytes[..4].try_into().unwrap()) as usize);
            bytes = &bytes[4..];
        }

        if (size_of::<usize>() > 2) && bytes.len() >= 2 {
            hash.add_to_hash(u16::from_ne_bytes(bytes[..2].try_into().unwrap()) as usize);
            bytes = &bytes[2..];
        }

        if (size_of::<usize>() > 1) && bytes.len() >= 1 {
            hash.add_to_hash(bytes[0] as usize);
        }

        self.hash = hash.hash;
    }

    #[inline]
    fn write_u8(&mut self, value: u8) {
        self.add_to_hash(value as usize);
    }

    #[inline]
    fn write_u16(&mut self, value: u16) {
        self.add_to_hash(value as usize);
    }

    #[inline]
    fn write_u32(&mut self, value: u32) {
        self.add_to_hash(value as usize);
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn write_u64(&mut self, value: u64) {
        self.add_to_hash(value as usize);
        self.add_to_hash((value >> 32) as usize);
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn write_u64(&mut self, value: u64) {
        self.add_to_hash(value as usize);
    }

    #[inline]
    fn write_usize(&mut self, value: usize) {
        self.add_to_hash(value);
    }
}

pub type PascalcHashMap<K, V> = HashMap<K, V, BuildHasherDefault<PascalcHasher>>;
pub type PascalcHashSet<V> = HashSet<V, BuildHasherDefault<PascalcHasher>>;
