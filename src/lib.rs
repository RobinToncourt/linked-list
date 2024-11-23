#![allow(dead_code)]

mod linked_list;
mod linked_list_iterator;

trait List<T> {
    fn insert(&mut self, index: usize, element: T);
    fn len(&self) -> usize;
    fn pop(&mut self) -> Option<T>;
    fn push(&mut self, value: T);
    fn remove(&mut self, index: usize) -> T;
    fn get(&self, index: usize) -> Option<&T>;
    fn split_off(&mut self, at: usize) -> Self;
}
