use crate::gvec::Gvec;
use core::panic;
use rand::rngs::StdRng;
use std::{
    fmt::{self, Debug, Display},
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
    pub const WORD_SIZE_MASK: usize = Self::WORD_SIZE - 1;

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
    pub fn from_elem(len: usize, val: bool) -> Self {
        if len == 0 {
            return Self::default();
        }
        let v = if val { u64::MAX } else { 0 };
        let mut bits = Gvec::from(vec![v; len / Self::WORD_SIZE]);
        let mut last_len = len & Self::WORD_SIZE_MASK;
        if last_len == 0 {
            last_len = 64;
        } else {
            bits.push(if val { (1 << last_len) - 1 } else { 0 });
        }
        Self { bits, last_len }
    }

    pub fn from_usize(len: usize, v: usize) -> Self {
        if len == 0 {
            return Self::default();
        }
        let mut res = Self::zero(len);
        res.bits[0] = v as u64;
        res.mask_last();
        res
    }

    pub fn to_usize(&self) -> usize {
        if self.is_empty() {
            return 0;
        }
        self.bits[0usize] as usize
    }

    #[inline]
    pub fn len(&self) -> usize {
        (self.bits.len() - 1) * 64 + self.last_len
    }

    #[inline]
    pub fn word_len(&self) -> usize {
        let mut res = self.bits.len();
        if self.last_len == 0 {
            res -= 1
        }
        res
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn clear(&mut self) {
        *self = Self::default();
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
    fn mask_last(&mut self) {
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
    #[inline]
    pub fn push_word(&mut self, word: u64) {
        if self.last_len == 0 {
            let l = self.bits.len() - 1;
            self.bits[l] = word;
            self.last_len = 64;
        } else if self.last_len == 64 {
            self.bits.push(word);
        } else {
            panic!();
        }
    }

    #[inline]
    pub fn zero(len: usize) -> Self {
        debug_assert!(len > 0);
        Self::from_elem(len, false)
    }

    #[inline]
    pub fn ones(len: usize) -> Self {
        debug_assert!(len > 0);
        Self::from_elem(len, true)
    }

    #[inline]
    pub fn one(len: usize) -> Self {
        debug_assert!(len > 0);
        let mut r = Self::from_elem(len, false);
        r.set(0, true);
        r
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        debug_assert!(!self.is_empty());
        for i in 0..self.word_len() {
            if self.bits[i] != 0 {
                return false;
            }
        }
        true
    }

    #[inline]
    pub fn is_one(&self) -> bool {
        debug_assert!(!self.is_empty());
        if self.bits[0] != 1u64 {
            return false;
        }
        for i in 1..self.word_len() {
            if self.bits[i] != 0 {
                return false;
            }
        }
        true
    }

    #[inline]
    pub fn is_ones(&self) -> bool {
        debug_assert!(!self.is_empty());
        let wl = self.word_len();
        for i in 0..wl - 1 {
            if self.bits[i] != u64::MAX {
                return false;
            }
        }
        let mask = if self.last_len == 64 {
            u64::MAX
        } else {
            (1u64 << self.last_len) - 1
        };
        self.bits[wl - 1] == mask
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter {
            bv: self,
            start: 0,
            end: self.len(),
        }
    }
}

pub struct Iter<'a> {
    bv: &'a BitVec,
    start: usize,
    end: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let res = self.bv.get(self.start);
            self.start += 1;
            Some(res)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.end - self.start;
        (len, Some(len))
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            self.end -= 1;
            Some(self.bv.get(self.end))
        } else {
            None
        }
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {}

impl<'a> IntoIterator for &'a BitVec {
    type Item = bool;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct IntoIter {
    bv: BitVec,
    start: usize,
    end: usize,
}

impl Iterator for IntoIter {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let res = self.bv.get(self.start);
            self.start += 1;
            Some(res)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.end - self.start;
        (len, Some(len))
    }
}

impl DoubleEndedIterator for IntoIter {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            self.end -= 1;
            Some(self.bv.get(self.end))
        } else {
            None
        }
    }
}

impl ExactSizeIterator for IntoIter {}

impl IntoIterator for BitVec {
    type Item = bool;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        IntoIter {
            bv: self,
            start: 0,
            end: len,
        }
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
        for i in 0..self.word_len() {
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
        for &bit in self.bits.iter().take(self.word_len()) {
            bit.hash(state);
        }
        (self.last_len & Self::WORD_SIZE_MASK).hash(state);
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
        res.mask_last();
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
        res.mask_last();
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
        self.mask_last();
    }
}

impl<'a, I: IntoIterator<Item = &'a bool>> From<I> for BitVec {
    #[inline]
    fn from(value: I) -> Self {
        let mut r = Self::new();
        for x in value.into_iter() {
            r.push(*x);
        }
        r
    }
}

impl Extend<bool> for BitVec {
    fn extend<T: IntoIterator<Item = bool>>(&mut self, iter: T) {
        for x in iter {
            self.push(x);
        }
    }
}

impl<'a> Extend<&'a bool> for BitVec {
    fn extend<T: IntoIterator<Item = &'a bool>>(&mut self, iter: T) {
        for x in iter {
            self.push(*x);
        }
    }
}

impl Debug for BitVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "[]");
        }
        let mut s = String::new();
        for i in (0..self.len()).rev() {
            if self.get(i) {
                s.push('1');
            } else {
                s.push('0');
            }
        }
        write!(f, "{s}")
    }
}

impl Display for BitVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl fmt::LowerHex for BitVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "[]");
        }
        write!(f, "0x")?;
        if self.is_zero() {
            return write!(f, "0");
        }
        let wl = self.word_len();
        write!(f, "{:x}", self.bits[wl - 1])?;
        for i in (0..wl - 1).rev() {
            write!(f, "{:016x}", self.bits[i])?;
        }
        Ok(())
    }
}

impl fmt::UpperHex for BitVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "[]");
        }
        write!(f, "0x")?;
        if self.is_zero() {
            return write!(f, "0");
        }
        let wl = self.word_len();
        write!(f, "{:X}", self.bits[wl - 1])?;
        for i in (0..wl - 1).rev() {
            write!(f, "{:016X}", self.bits[i])?;
        }
        Ok(())
    }
}

impl fmt::Binary for BitVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::GHashSet;

    #[test]
    fn test0() {
        let v = [true, false, true, false, true];
        let bv = BitVec::from(&v);
        for i in 0..bv.len() {
            assert_eq!(bv.get(i), v[i]);
        }
    }

    #[test]
    fn test1() {
        let bv = BitVec::from_elem(0, true);
        assert!(bv.is_empty());
    }

    #[test]
    fn test2() {
        for w in 1..200 {
            let one = BitVec::one(w);
            assert!(one.is_one());
            let ones = BitVec::ones(w);
            assert!(ones.is_ones());
            let zero = BitVec::zero(w);
            assert!(zero.is_zero());
        }
    }

    #[test]
    fn test3() {
        let v = 12345;
        let bv = BitVec::from_usize(64, v);
        assert_eq!(bv.to_usize(), v);
        let bv2 = BitVec::from_usize(10, v);
        assert_eq!(bv2.to_usize(), v & ((1 << 10) - 1));
    }

    #[test]
    fn test4() {
        let v = [true, false, true, true, false];
        let bv = BitVec::from(&v);
        let mut iter = bv.iter();
        for &val in &v {
            assert_eq!(iter.next(), Some(val));
        }
        assert_eq!(iter.next(), None);
        for (i, val) in bv.into_iter().enumerate() {
            assert_eq!(val, v[i]);
        }
    }

    #[test]
    fn test_fmt() {
        let z = BitVec::new();
        assert_eq!(format!("{}", z), "[]");
        assert_eq!(format!("{:x}", z), "[]");
        assert_eq!(format!("{:X}", z), "[]");

        let v = 12345;
        let bv = BitVec::from_usize(64, v);
        let s = format!("{}", bv);
        assert_eq!(s.len(), 64);
        assert!(s.ends_with("11000000111001"));

        assert_eq!(format!("{:x}", bv), "0x3039");
        assert_eq!(format!("{:X}", bv), "0x3039");

        let mut bv_large = BitVec::zero(128);
        bv_large.bits[0] = u64::MAX;
        bv_large.bits[1] = 1_u64;

        assert_eq!(format!("{:x}", bv_large), "0x1ffffffffffffffff");
        assert_eq!(format!("{:X}", bv_large), "0x1FFFFFFFFFFFFFFFF");

        let s_large = format!("{}", bv_large);
        assert_eq!(s_large.len(), 128);
        let expected_suffix = "1".repeat(65);
        assert!(s_large.ends_with(&expected_suffix));
        assert!(s_large.starts_with('0'));
    }

    #[test]
    fn test_extend() {
        let mut bv = BitVec::new();
        bv.extend([true, false, true]);
        assert_eq!(bv.len(), 3);
        assert_eq!(bv.get(0), true);
        assert_eq!(bv.get(1), false);
        assert_eq!(bv.get(2), true);

        bv.extend(&[false, true]);
        assert_eq!(bv.len(), 5);
        assert_eq!(bv.get(3), false);
        assert_eq!(bv.get(4), true);
    }

    #[test]
    fn test_double_ended_iter() {
        let v = [true, false, true, true, false, true, false, false];
        let bv = BitVec::from(&v);

        let mut iter = bv.iter();
        let mut v_iter = v.iter().copied();
        while v_iter.len() > 0 {
            if v_iter.len() % 2 == 0 {
                assert_eq!(iter.next(), v_iter.next());
            } else {
                assert_eq!(iter.next_back(), v_iter.next_back());
            }
        }
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);

        let mut iter = bv.clone().into_iter();
        let mut v_iter = v.iter().copied();
        while v_iter.len() > 0 {
            if v_iter.len() % 2 != 0 {
                assert_eq!(iter.next(), v_iter.next());
            } else {
                assert_eq!(iter.next_back(), v_iter.next_back());
            }
        }
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_hash_eq() {
        let a = BitVec::ones(64);
        let b = BitVec {
            bits: Gvec::from([u64::MAX, 0]),
            last_len: 0,
        };
        assert!(a == b);
        let mut s = GHashSet::new();
        s.insert(a);
        s.insert(b);
        assert!(s.len() == 1);
    }
}
