use crate::{
    grc::Grc,
    gvec::Gvec,
    hash::{GHashMap, GHashSet},
};
use std::{
    mem::swap,
    ops::{Index, IndexMut},
};

pub struct Gallocator<T> {
    data: Gvec<T>,
    removed: Grc<GHashSet<u32>>,
}

impl<T> Gallocator<T> {
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn alloc(&mut self, v: T) -> u32 {
        self.data.push(v);
        self.data.len() - 1
    }

    #[inline]
    pub fn dealloc(&mut self, idx: u32) {
        self.removed.insert(idx);
    }

    #[inline]
    pub fn gc(&mut self) -> GHashMap<u32, u32> {
        let mut map = GHashMap::new();
        let mut data = Gvec::new();
        swap(&mut self.data, &mut data);
        for (i, d) in data.into_iter().enumerate() {
            let i = i as u32;
            if self.removed.contains(&i) {
                continue;
            }
            map.insert(i, self.alloc(d));
        }
        self.removed.clear();
        map
    }

    pub fn get_removed(&self) -> Grc<GHashSet<u32>> {
        self.removed.clone()
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
        Self {
            data: Gvec::new(),
            removed: Grc::new(GHashSet::new()),
        }
    }
}
