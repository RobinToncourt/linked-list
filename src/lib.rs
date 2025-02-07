#![allow(dead_code)]

pub mod linked_list;
pub mod linked_list_iterator;

pub trait List<T> {
    fn insert(&mut self, index: usize, element: T);
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn pop(&mut self) -> Option<T>;
    fn push(&mut self, value: T);
    fn remove(&mut self, index: usize) -> T;
    fn get(&self, index: usize) -> Option<&T>;
    #[must_use]
    fn split_off(&mut self, at: usize) -> Self;
    fn is_sorted(&self) -> bool
    where
        T: PartialOrd;
    fn sort(&mut self)
    where
        T: Ord;
}
