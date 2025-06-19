use crate::gvec::Gvec;
use rand::rngs::StdRng;
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

#[derive(Clone)]
pub struct BitVec {
    bits: Gvec<u64>,
    last_len: usize,
}

impl BitVec {
    pub const WORD_SIZE: usize = 64;

    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn new_rand(num_word: usize, rng: &mut StdRng) -> Self {
        Self {
            bits: Gvec::new_rand(num_word, rng),
            last_len: 64,
        }
    }

    #[inline]
    pub fn new_with(len: usize, val: bool) -> Self {
        let v = if val { u64::MAX } else { 0 };
        let mut bits = Gvec::from(vec![v; len / Self::WORD_SIZE]);
        let mut last_len = len % Self::WORD_SIZE;
        if last_len == 0 {
            last_len = 64;
        } else {
            bits.push(if val { (1 << last_len) - 1 } else { 0 });
        }
        Self { bits, last_len }
    }

    #[inline]
    pub fn len(&self) -> usize {
        (self.bits.len() - 1) * 64 + self.last_len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn get(&self, index: usize) -> bool {
        debug_assert!(index < self.len());
        let word_index = index / 64;
        let bit_index = index % 64;
        let mask = 1 << bit_index;
        (self.bits[word_index] & mask) != 0
    }

    #[inline]
    pub fn set(&mut self, index: usize, val: bool) {
        debug_assert!(index < self.len());
        let word_index = index / 64;
        let bit_index = index % 64;
        let mask = 1 << bit_index;
        if val {
            self.bits[word_index] |= mask;
        } else {
            self.bits[word_index] &= !mask;
        }
    }

    #[inline]
    #[allow(unused)]
    fn last_word(&self) -> &u64 {
        unsafe { self.bits.last().unwrap_unchecked() }
    }

    #[inline]
    fn last_word_mut(&mut self) -> &mut u64 {
        unsafe { self.bits.last_mut().unwrap_unchecked() }
    }

    #[inline]
    fn mast_last(&mut self) {
        if self.last_len == 64 {
            return;
        }
        let mask = (1 << self.last_len) - 1;
        *self.last_word_mut() &= mask;
    }

    #[inline]
    pub fn push(&mut self, bit: bool) {
        if self.last_len == 64 {
            self.bits.push(0);
            self.last_len = 0;
        }
        let mask = 1 << self.last_len;
        let x = self.last_word_mut();
        if bit {
            *x |= mask;
        } else {
            *x &= !mask;
        }
        self.last_len += 1;
    }
}

impl Default for BitVec {
    #[inline]
    fn default() -> Self {
        Self {
            bits: Gvec::from([0]),
            last_len: 0,
        }
    }
}

impl PartialEq for BitVec {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for i in 0..self.bits.len() {
            if self.bits[i] != other.bits[i] {
                return false;
            }
        }
        true
    }
}

impl Eq for BitVec {}

impl Hash for BitVec {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for &bit in self.bits.iter() {
            bit.hash(state);
        }
        self.last_len.hash(state);
    }
}

impl Not for &BitVec {
    type Output = BitVec;

    #[inline]
    fn not(self) -> BitVec {
        let mut res = self.clone();
        for r in res.bits.iter_mut() {
            *r = !*r;
        }
        res.mast_last();
        res
    }
}

impl BitAnd for BitVec {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        assert!(self.len() == rhs.len());
        Self {
            bits: self
                .bits
                .iter()
                .zip(rhs.bits.iter())
                .map(|(s, r)| s & r)
                .collect(),
            last_len: self.last_len,
        }
    }
}

impl BitOr for BitVec {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        assert!(self.len() == rhs.len());
        Self {
            bits: self
                .bits
                .iter()
                .zip(rhs.bits.iter())
                .map(|(s, r)| s | r)
                .collect(),
            last_len: self.last_len,
        }
    }
}

impl BitXor for BitVec {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        assert!(self.len() == rhs.len());
        let mut res = Self {
            bits: self
                .bits
                .iter()
                .zip(rhs.bits.iter())
                .map(|(s, r)| s ^ r)
                .collect(),
            last_len: self.last_len,
        };
        res.mast_last();
        res
    }
}

impl BitAndAssign<&BitVec> for BitVec {
    #[inline]
    fn bitand_assign(&mut self, rhs: &Self) {
        assert!(self.len() == rhs.len());
        for (s, r) in self.bits.iter_mut().zip(rhs.bits.iter()) {
            *s &= r;
        }
    }
}

impl BitOrAssign<&BitVec> for BitVec {
    #[inline]
    fn bitor_assign(&mut self, rhs: &BitVec) {
        assert!(self.len() == rhs.len());
        for (s, r) in self.bits.iter_mut().zip(rhs.bits.iter()) {
            *s |= r;
        }
    }
}

impl BitXorAssign<&BitVec> for BitVec {
    #[inline]
    fn bitxor_assign(&mut self, rhs: &BitVec) {
        assert!(self.len() == rhs.len());
        for (s, r) in self.bits.iter_mut().zip(rhs.bits.iter()) {
            *s ^= r;
        }
        self.mast_last();
    }
}

impl Display for BitVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in 0..self.len() {
            if self.get(i) {
                s.push('1');
            } else {
                s.push('0');
            }
        }
        write!(f, "{s}")
    }
}

impl Debug for BitVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in 0..self.len() {
            if self.get(i) {
                s.push('1');
            } else {
                s.push('0');
            }
        }
        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut bv = BitVec::new();
        let v = [true, false, true, false, true];
        for &x in &v {
            bv.push(x);
        }
        for i in 0..bv.len() {
            assert_eq!(bv.get(i), v[i]);
        }
    }
}
