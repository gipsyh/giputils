use std::{
    ops::{Index, IndexMut},
    slice,
};

#[derive(Default, Debug)]
pub struct Gvec<T> {
    data: Vec<T>,
}

impl<T> Gvec<T> {
    #[inline]
    pub fn len(&self) -> u32 {
        self.data.len() as u32
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    pub fn push(&mut self, x: T) {
        self.data.push(x)
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    #[inline]
    pub fn swap_remove(&mut self, index: u32) {
        self.data.swap_remove(index as usize);
    }

    #[inline]
    pub fn iter(&self) -> slice::Iter<T> {
        self.data.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> slice::IterMut<T> {
        self.data.iter_mut()
    }
}

impl<T: Default> Gvec<T> {
    #[inline]
    pub fn reserve(&mut self, size: usize) {
        self.data.resize_with(size, Default::default);
    }
}

impl<T> Index<u32> for Gvec<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: u32) -> &Self::Output {
        #[cfg(feature = "no_bound_check")]
        unsafe {
            self.data.get_unchecked(index as usize)
        }
        #[cfg(not(feature = "no_bound_check"))]
        &self.data[index as usize]
    }
}

impl<T> IndexMut<u32> for Gvec<T> {
    #[inline]
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        #[cfg(feature = "no_bound_check")]
        unsafe {
            self.data.get_unchecked_mut(index as usize)
        }
        #[cfg(not(feature = "no_bound_check"))]
        &mut self.data[index as usize]
    }
}
