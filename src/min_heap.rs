use std::cmp::Ord;

/// The heap abstract data structure.
pub trait MinHeap<T: Ord> {
    /// Insert a new value in the heap.
    fn push(&mut self, value: T);

    /// Returns an imutable borrow to the smallest element in the heap.
    /// Return None if the heap is empty.
    fn top(&self) -> Option<&T>;

    /// Returns ownership of the smallest element in the heap and deletes it from the heap.
    /// Return NOne if the heap is empty.
    fn pop(&mut self) -> Option<T>;

    /// Returns the number of elements in the heap.
    fn len(&self) -> usize;

    /// Returns true if the heap is empty.
    fn is_empty(&self) -> bool;
}
