use ahash::{HashMap, HashSet, RandomState};
use std::ops::{Deref, DerefMut};

pub struct GHashSet<T> {
    h: HashSet<T>,
}

impl<T> GHashSet<T> {
    pub fn new() -> Self {
        let hb = RandomState::with_seeds(0, 0, 0, 0);
        Self {
            h: HashSet::with_hasher(hb),
        }
    }
}

impl<T> Deref for GHashSet<T> {
    type Target = HashSet<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.h
    }
}

impl<T> DerefMut for GHashSet<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.h
    }
}

impl<T> Default for GHashSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct GHashMap<K, V> {
    h: HashMap<K, V>,
}

impl<K, V> GHashMap<K, V> {
    pub fn new() -> Self {
        let hb = RandomState::with_seeds(0, 0, 0, 0);
        Self {
            h: HashMap::with_hasher(hb),
        }
    }
}

impl<K, V> Deref for GHashMap<K, V> {
    type Target = HashMap<K, V>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.h
    }
}

impl<K, V> DerefMut for GHashMap<K, V> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.h
    }
}

impl<K, V> Default for GHashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
