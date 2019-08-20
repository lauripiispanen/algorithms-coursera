use std::cmp::Ordering;
use std::collections::VecDeque;

pub struct BinHeap<T: Ord> {
    ordering: Ordering,
    values: VecDeque<T>
}

impl<T: Ord> BinHeap<T> {
    pub fn insert(&mut self, t: T) {
        self.values.push_back(t);
        let mut i = self.values.len() - 1;
        while i > 0 {
            let b_i = i;
            let b = &self.values[b_i];
            let a_i = BinHeap::<T>::parent_of(i);
            let a = &self.values[a_i];

            if b.cmp(a) == self.ordering {
                self.values.swap(a_i, b_i);
            }
            i = a_i;
        }
    }

    fn parent_of(i: usize) -> usize {
        (i - 1) / 2
    }

    pub fn new_minheap() -> BinHeap<T> {
        BinHeap {
            values: VecDeque::new(),
            ordering: Ordering::Less
        }
    }

    pub fn new_maxheap() -> BinHeap<T> {
        BinHeap {
            values: VecDeque::new(),
            ordering: Ordering::Greater
        }
    }

    fn min_heapify(&mut self, from_i: usize) -> usize {
        let left_i = from_i * 2 + 1;
        let right_i = from_i * 2 + 2;
        let mut smallest_i = from_i;

        if left_i < self.values.len() && self.values[left_i].cmp(&self.values[smallest_i]) == self.ordering {
            smallest_i = left_i;
        }

        if right_i < self.values.len() && self.values[right_i].cmp(&self.values[smallest_i]) == self.ordering {
            smallest_i = right_i;
        }

        if smallest_i != from_i {
            self.values.swap(smallest_i, from_i);
        }
        return smallest_i;
    }

    pub fn extract(&mut self) -> Option<T> {
        if (self.values.len() > 1) {
            let v = self.values.pop_front();
            let b = self.values.pop_back().unwrap();
            self.values.push_front(b);
            let mut i = 0;

            // workaround for lack of tail recursion
            loop {
                let new_i = self.min_heapify(i);
                if new_i == i {
                    break;
                }
                i = new_i;
            }
            return v;
        } else {
            return self.values.pop_front();
        }
    }

    pub fn peek(&mut self) -> Option<&T> {
        return self.values.front();
    }

    pub fn is_empty(&self) -> bool {
        return self.values.is_empty();
    }

    pub fn len(&self) -> usize {
        return self.values.len();
    }
}

#[cfg(test)]
mod tests {

    extern crate rand;

    use rand::{thread_rng, seq::SliceRandom};

    #[test]
    fn test_insert() {
        let mut h = super::BinHeap::new_minheap();
        h.insert(3);
        h.insert(1);
        h.insert(7);
        h.insert(2);
        h.insert(-3);

        assert_eq!(h.values, &[-3,1,7,3,2])
    }

    #[test]
    fn test_extract() {
        let mut h = super::BinHeap::new_minheap();
        h.insert(3);
        h.insert(1);
        h.insert(7);
        h.insert(2);
        h.insert(-3);

        let mut v = Vec::new();
        assert_eq!(-3, *h.peek().unwrap());
        while !h.is_empty() {
            v.push(h.extract().unwrap());
        }
        assert_eq!(v, &[-3,1,2,3,7])
    }

    #[test]
    fn test_maxheap() {
        let mut h = super::BinHeap::new_maxheap();
        h.insert(3);
        h.insert(1);
        h.insert(7);
        h.insert(2);
        h.insert(-3);

        let mut v = Vec::new();
        while !h.is_empty() {
            println!("{:?}", h.values);
            v.push(h.extract().unwrap());
        }
        assert_eq!(v, &[7,3,2,1,-3])
    }

    #[test]
    fn test_minheap_big() {
        let mut h = super::BinHeap::new_minheap();
        let mut nums: Vec<u32> = (1..=10000).collect();

        nums.shuffle(&mut thread_rng());

        for n in nums {
            h.insert(n);
        }

        for n in 1..=10000 {
            assert_eq!(n, h.extract().unwrap());
        }
    }

}