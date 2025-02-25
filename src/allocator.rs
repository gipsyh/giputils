use crate::{gvec::Gvec, hash::GHashMap};
use std::{
    mem::swap,
    ops::{Deref, DerefMut, Index, IndexMut},
};

pub struct GallocElem<T> {
    e: T,
    removed: bool,
}

impl<T> Deref for GallocElem<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.e
    }
}

impl<T> DerefMut for GallocElem<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.e
    }
}

impl<T> GallocElem<T> {
    #[inline]
    pub fn new(e: T) -> Self {
        Self { e, removed: false }
    }

    #[inline]
    pub fn take(self) -> T {
        self.e
    }

    #[inline]
    pub fn is_removed(&self) -> bool {
        self.removed
    }
}

pub struct Gallocator<T> {
    data: Gvec<GallocElem<T>>,
}

impl<T> Gallocator<T> {
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn alloc(&mut self, v: T) -> u32 {
        self.data.push(GallocElem::new(v));
        self.data.len() - 1
    }

    #[inline]
    pub fn dealloc(&mut self, idx: u32) {
        self.data[idx].removed = true;
    }

    #[inline]
    pub fn is_removed(&self, idx: u32) -> bool {
        self.data[idx].is_removed()
    }

    #[inline]
    pub fn gc(&mut self) -> GHashMap<u32, u32> {
        let mut map = GHashMap::new();
        let mut data = Gvec::new();
        swap(&mut self.data, &mut data);
        for (i, d) in data.into_iter().enumerate() {
            let i = i as u32;
            if d.is_removed() {
                continue;
            }
            map.insert(i, self.alloc(d.take()));
        }
        map
    }
}

impl<T> Index<u32> for Gallocator<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: u32) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<u32> for Gallocator<u32> {
    #[inline]
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Default for Gallocator<T> {
    #[inline]
    fn default() -> Self {
        Self { data: Gvec::new() }
    }
}
