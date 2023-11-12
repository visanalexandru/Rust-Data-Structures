use crate::MinHeap;
use std::cmp::Ord;

/// A binary heap is an implementation of a min-heap using a binary tree.
pub struct BinaryHeap<T: Ord> {
    values: Vec<T>,
}

fn left_son(index: usize) -> usize {
    2 * index + 1
}
fn right_son(index: usize) -> usize {
    2 * index + 2
}
fn father(index: usize) -> Option<usize> {
    if index == 0 {
        None
    } else {
        Some((index + 1) / 2 - 1)
    }
}

impl<T: Ord> BinaryHeap<T> {
    /// Returns an empty binary heap.
    pub fn new() -> Self {
        BinaryHeap { values: vec![] }
    }
}

impl<T: Ord> MinHeap<T> for BinaryHeap<T> {
    fn push(&mut self, val: T) {
        self.values.push(val);
        let mut current = self.values.len() - 1;

        loop {
            let f = match father(current) {
                Some(node) => node,
                None => break,
            };
            if self.values[current] >= self.values[f] {
                break;
            }
            self.values.swap(current, f);
            current = f;
        }
    }

    fn top(&self) -> Option<&T> {
        self.values.get(0)
    }

    fn pop(&mut self) -> Option<T> {
        if self.values.is_empty() {
            return None;
        }

        if self.values.len() == 1 {
            return self.values.pop();
        }

        let mut last = self.values.len() - 1;
        self.values.swap(0, last);
        let top = self.values.pop();
        last -= 1;

        let mut current_node: usize = 0;

        loop {
            let left_son = left_son(current_node);
            let right_son = right_son(current_node);

            // Stop if the current node doesn't have any children.
            if left_son > last {
                break;
            }

            // Find the smallest child.
            let mut smallest = left_son;
            if right_son <= last && self.values[right_son] < self.values[left_son] {
                smallest = right_son;
            }

            // Stop if the current node is where it's supposed to be.
            if self.values[current_node] <= self.values[smallest] {
                break;
            }

            self.values.swap(current_node, smallest);
            current_node = smallest;
        }

        top
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Standard;
    use rand::prelude::*;
    use std::cmp::Reverse;
    use std::collections;

    enum Operation<T> {
        Insert(T),
        Pop(),
    }

    #[test]
    fn empty_heap_is_empty() {
        let h = BinaryHeap::<usize>::new();
        assert!(h.is_empty());
    }

    #[test]
    fn nonempty_heap_is_not_empty() {
        let mut h = BinaryHeap::<usize>::new();
        h.push(10);
        assert!(!h.is_empty());
    }

    #[test]
    fn empty_heap_has_zero_len() {
        let h = BinaryHeap::<usize>::new();
        assert_eq!(h.len(), 0);
    }

    #[test]
    fn insert_increases_heap_len() {
        let mut h = BinaryHeap::<usize>::new();
        h.push(2);
        h.push(10);
        h.push(4);
        h.push(100);
        assert_eq!(h.len(), 4);
    }

    #[test]
    fn pop_decreases_heap_len() {
        let mut h = BinaryHeap::<usize>::new();
        h.push(2);
        h.push(10);
        h.push(4);
        h.push(100);

        for _ in 0..4 {
            h.pop();
        }
        assert_eq!(h.len(), 0);
    }

    #[test]
    fn top_returns_none_on_empty_heap() {
        let h = BinaryHeap::<usize>::new();
        assert!(h.top().is_none());
    }

    #[test]
    fn top_returns_minimum_element() {
        let mut h = BinaryHeap::new();
        let values = vec![1, 10, 3, -4123, 34, 100, 124];
        for v in values {
            h.push(v);
        }
        assert_eq!(*h.top().unwrap(), -4123);
    }

    #[test]
    fn top_changes_on_pop() {
        let mut h = BinaryHeap::new();
        let values = vec![1, 10, 3, -4123, 34, 100, 124];

        let mut ordered = values.clone();
        ordered.sort();

        for v in values.iter() {
            h.push(*v);
        }

        for i in 0..values.len() - 1 {
            h.pop();
            assert_eq!(*h.top().unwrap(), ordered[i + 1]);
        }
    }

    #[test]
    fn pop_returns_none_on_empty_heap() {
        let mut h = BinaryHeap::<usize>::new();
        assert!(h.pop().is_none())
    }

    #[test]
    fn pop_returns_and_erases_minimum_element() {
        let mut h = BinaryHeap::new();
        let values = vec![1, 10, 3, -4123, 34, 100, 124];

        let mut ordered = values.clone();
        ordered.sort();

        for v in values.iter() {
            h.push(*v);
        }

        for i in 0..values.len() {
            assert_eq!(h.pop().unwrap(), ordered[i]);
        }
    }

    // The operation gen strategy.
    fn strategy<T>(rng: &mut ThreadRng, heap_size: usize) -> Operation<T>
    where
        Standard: Distribution<T>,
    {
        if heap_size > 0 {
            let should_pop = rng.gen_bool(0.3);
            if should_pop {
                return Operation::Pop();
            }
        }
        Operation::Insert(rng.gen())
    }

    // Random fuzz testing.
    #[test]
    #[ignore]
    fn fuzz() {
        let num_operations: usize = 5000000;
        let mut rng = thread_rng();

        let mut ours = BinaryHeap::<i32>::new();
        let mut theirs = collections::BinaryHeap::<Reverse<i32>>::new();

        for _ in 0..num_operations {
            let op = strategy(&mut rng, ours.len());
            match op {
                Operation::Insert(val) => {
                    ours.push(val);
                    theirs.push(Reverse(val));
                }
                Operation::Pop() => {
                    assert_eq!(ours.pop().unwrap(), theirs.pop().unwrap().0);
                }
            }
        }
    }
}
