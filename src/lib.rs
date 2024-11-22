#![allow(dead_code)]

mod linked_list;

trait List<T> {
    fn insert(&mut self, index: usize, element: T);
    fn len(&self) -> usize;
    fn pop(&mut self) -> Option<T>;
    fn push(&mut self, value: T);
    fn remove(&mut self, index: usize) -> T;
    fn get(&self, index: usize) -> Option<&T>;
    fn into_vec(self) -> Vec<T>;
}
