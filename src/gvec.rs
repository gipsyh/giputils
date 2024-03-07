use std::{
    ops::{Deref, DerefMut, Index, IndexMut},
    vec,
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
    pub fn swap_remove(&mut self, index: u32) {
        self.data.swap_remove(index as usize);
    }
}

impl<T: Default> Gvec<T> {
    #[inline]
    pub fn reserve(&mut self, size: u32) {
        if self.len() < size {
            self.data.resize_with(size as usize, Default::default);
        }
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

impl<T> IntoIterator for Gvec<T> {
    type Item = T;

    type IntoIter = vec::IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T> Deref for Gvec<T> {
    type Target = Vec<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Gvec<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
