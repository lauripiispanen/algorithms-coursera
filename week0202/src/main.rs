use std::cmp::Ordering;
use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
}

struct MinHeap<T: Ord> {
    values: VecDeque<T>
}

impl<T: Ord> MinHeap<T> {
    fn insert(&mut self, t: T) {
        self.values.push_back(t);
        let mut i = self.values.len() - 1;
        while i > 0 {
            let b_i = i;
            let b = &self.values[b_i];
            let a_i = MinHeap::<T>::parent_of(i);
            let a = &self.values[a_i];

            if a.cmp(b) == Ordering::Greater {
                self.values.swap(a_i, b_i);
            }
            i = a_i;
        }
    }

    fn parent_of(i: usize) -> usize {
        (i - 1) / 2
    }

    fn new() -> MinHeap<T> {
        MinHeap {
            values: VecDeque::new()
        }
    }

    fn min_heapify(&mut self, from_i: usize) -> usize {
        let left_i = from_i * 2 + 1;
        let right_i = from_i * 2 + 2;
        let mut smallest_i = from_i;

        if left_i < self.values.len() && self.values[left_i] < self.values[smallest_i] {
            smallest_i = left_i;
        }

        if right_i < self.values.len() && self.values[right_i] < self.values[smallest_i] {
            smallest_i = right_i;
        }

        if smallest_i != from_i {
            self.values.swap(smallest_i, from_i);
        }
        return smallest_i;
    }

    fn extract(&mut self) -> Option<T> {
        let v = self.values.pop_front();
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
    }

    fn is_empty(&self) -> bool {
        return self.values.is_empty();
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_insert() {
        let mut h = super::MinHeap::new();
        h.insert(3);
        h.insert(1);
        h.insert(7);
        h.insert(2);
        h.insert(-3);

        assert_eq!(h.values, &[-3,1,7,3,2])
    }

    #[test]
    fn test_extract() {
        let mut h = super::MinHeap::new();
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
        assert_eq!(v, &[-3,1,2,3,7])
    }

}