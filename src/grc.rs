use std::{
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
    rc::Rc,
};

#[derive(Debug, Clone, Default)]
pub struct Grc<T> {
    rc: Rc<T>,
}

impl<T> Grc<T> {
    #[inline]
    pub fn new(v: T) -> Self {
        Self { rc: Rc::new(v) }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        Rc::as_ptr(&self.rc)
    }

    #[inline]
    pub fn from_ptr(p: *const T) -> Self {
        let rc = unsafe { Rc::from_raw(p) };
        Self { rc }
    }

    #[inline]
    pub fn count(&self) -> usize {
        Rc::strong_count(&self.rc)
    }

    #[inline]
    pub fn increment_count(&self) {
        unsafe { Rc::increment_strong_count(self.as_ptr()) }
    }
}

impl<T> Deref for Grc<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.rc.deref()
    }
}

impl<T> DerefMut for Grc<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { Rc::get_mut_unchecked(&mut self.rc) }
    }
}

impl<T> PartialEq for Grc<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Rc::as_ptr(&self.rc) == Rc::as_ptr(&other.rc)
    }
}

impl<T> Eq for Grc<T> {}

impl<T> Hash for Grc<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.rc).hash(state);
    }
}

unsafe impl<T> Sync for Grc<T> {}
