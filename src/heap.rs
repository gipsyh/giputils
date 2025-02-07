use crate::{gvec::Gvec, OptionU32};

#[derive(Default)]
pub struct BinaryHeap<T: Into<u32> + Copy> {
    heap: Gvec<T>,
    pos: Gvec<OptionU32>,
}

impl<T: Into<u32> + Copy> BinaryHeap<T> {
    #[inline]
    pub fn clear(&mut self) {
        for v in self.heap.iter().copied() {
            self.pos[v.into()] = OptionU32::NONE;
        }
        self.heap.clear();
    }

    #[inline]
    fn up<LGE: Fn(T, T) -> bool>(&mut self, v: T, lge: LGE) {
        let mut idx = match self.pos[v.into()] {
            OptionU32::NONE => return,
            idx => *idx,
        };
        while idx != 0 {
            let pidx = (idx - 1) >> 1;
            if lge(self.heap[pidx], v) {
                break;
            }
            self.heap[idx] = self.heap[pidx];
            *self.pos[self.heap[idx].into()] = idx;
            idx = pidx;
        }
        self.heap[idx] = v;
        *self.pos[v.into()] = idx;
    }

    #[inline]
    fn down<LGE: Fn(T, T) -> bool>(&mut self, mut idx: u32, lge: LGE) {
        let v = self.heap[idx];
        loop {
            let left = (idx << 1) + 1;
            if left >= self.heap.len() {
                break;
            }
            let right = left + 1;
            let child = if right < self.heap.len() && lge(self.heap[right], self.heap[left]) {
                right
            } else {
                left
            };
            if lge(v, self.heap[child]) {
                break;
            }
            self.heap[idx] = self.heap[child];
            *self.pos[self.heap[idx].into()] = idx;
            idx = child;
        }
        self.heap[idx] = v;
        *self.pos[v.into()] = idx;
    }

    #[inline]
    pub fn push<LGE: Fn(T, T) -> bool>(&mut self, v: T, lge: LGE) {
        self.pos.reserve(v.into() + 1);
        if self.pos[v.into()].is_some() {
            return;
        }
        let idx = self.heap.len();
        self.heap.push(v);
        *self.pos[v.into()] = idx;
        self.up(v, lge);
    }

    #[inline]
    pub fn pop<LGE: Fn(T, T) -> bool>(&mut self, lge: LGE) -> Option<T> {
        if self.heap.is_empty() {
            return None;
        }
        let value = self.heap[0];
        self.heap[0] = self.heap[self.heap.len() - 1];
        *self.pos[self.heap[0].into()] = 0;
        self.pos[value.into()] = OptionU32::NONE;
        self.heap.pop();
        if self.heap.len() > 1 {
            self.down(0, lge);
        }
        Some(value)
    }
}
