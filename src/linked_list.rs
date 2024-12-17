use std::ops::Index;
use std::ops::IndexMut;

use crate::linked_list_iterator::LinkedListIterator;
use crate::List;

const OUT_OF_BOUND: &str = "Index out of bound!";

#[derive(Debug)]
pub struct LinkedList<T> {
    value: Option<T>,
    next: Option<Box<LinkedList<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            value: None,
            next: None,
        }
    }

    pub fn pop_first(&mut self) -> Option<T> {
        let result = self.value.take();

        if self.next.is_some() {
            let mut next = self.next.take().unwrap();
            self.value = next.value.take();
            self.next = next.next.take();
        }

        result
    }

    fn is_last(&self) -> bool {
        self.next.is_none()
    }

    fn into_vec(mut self) -> Vec<T> {
        let mut result: Vec<T> = Vec::new();

        if self.value.is_some() {
            result.push(self.value.take().unwrap());
            if self.next.is_some() {
                result.append(&mut self.next.unwrap().into_vec());
            }
        }

        result
    }

    fn get_lowest_ref_mut_value(&mut self) -> Option<&mut T>
    where
        T: PartialOrd,
    {
        if self.value.is_none() {
            None
        } else if self.next.is_none() {
            self.value.as_mut()
        } else {
            let mut_value = self.value.as_mut().unwrap();
            let mut_next = self.next.as_mut().unwrap();

            let lowest = mut_next.get_lowest_ref_mut_value().unwrap();

            if mut_value < lowest {
                Some(mut_value)
            } else {
                Some(lowest)
            }
        }
    }
}

impl<T> List<T> for LinkedList<T> {
    fn insert(&mut self, index: usize, element: T) {
        if index == 0 {
            if self.value.is_some() {
                let value = self.value.take();
                let next = self.next.take();

                let new_linked_list = LinkedList { value, next };

                self.next = Some(Box::new(new_linked_list));
            }
            self.value = Some(element);
        } else if self.next.is_some() {
            self.next.as_mut().unwrap().insert(index - 1, element);
        } else if index == 1 {
            self.push(element);
        } else {
            panic!("{OUT_OF_BOUND}")
        }
    }

    fn len(&self) -> usize {
        if self.value.is_some() {
            if self.next.is_some() {
                1 + self.next.as_ref().unwrap().len()
            } else {
                1
            }
        } else {
            0
        }
    }

    fn is_empty(&self) -> bool {
        self.value.is_none()
    }

    fn pop(&mut self) -> Option<T> {
        if self.next.is_some() {
            if self.next.as_ref().unwrap().is_last() {
                self.next.take().unwrap().value
            } else {
                self.next.as_mut().unwrap().pop()
            }
        } else {
            self.value.take()
        }
    }

    fn push(&mut self, value: T) {
        if self.value.is_none() {
            self.value = Some(value);
        } else if self.next.is_some() {
            self.next.as_mut().unwrap().push(value);
        } else {
            let mut new_linked_list = Self::new();
            new_linked_list.push(value);
            self.next = Some(Box::new(new_linked_list));
        }
    }

    fn remove(&mut self, index: usize) -> T {
        if index == 0 {
            if self.value.is_some() {
                let output = self.value.take().unwrap();

                if self.next.is_some() {
                    let next = self.next.take().unwrap();
                    self.value = next.value;
                    self.next = next.next;
                }

                output
            } else {
                panic!("{OUT_OF_BOUND}")
            }
        } else if self.next.is_some() {
            self.next.as_mut().unwrap().remove(index - 1)
        } else {
            panic!("{OUT_OF_BOUND}")
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index == 0 {
            self.value.as_ref()
        } else if self.next.is_some() {
            self.next.as_ref().unwrap().get(index - 1)
        } else {
            None
        }
    }

    fn split_off(&mut self, at: usize) -> Self {
        if at == 0 {
            LinkedList {
                value: self.value.take(),
                next: self.next.take(),
            }
        } else if at == 1 {
            if self.next.is_some() {
                *self.next.take().unwrap()
            } else {
                Self::new()
            }
        } else if self.next.is_some() {
            self.next.as_mut().unwrap().split_off(at - 1)
        } else {
            panic!("{OUT_OF_BOUND}")
        }
    }

    fn is_sorted(&self) -> bool
    where
        T: PartialOrd,
    {
        if self.value.is_none() || self.next.is_none() {
            true
        } else {
            let ref_val = self.value.as_ref().unwrap();
            let ref_next = self.next.as_ref().unwrap();

            ref_val <= ref_next.value.as_ref().unwrap() && ref_next.is_sorted()
        }
    }

    fn sort(&mut self)
    where
        T: Ord,
    {
        if self.value.is_some()
            && self.next.is_some()
            && self.value.is_some()
            && self.next.is_some()
        {
            let mut_value = self.value.as_mut().unwrap();
            let mut_next = self.next.as_mut().unwrap();
            let lowest = mut_next.get_lowest_ref_mut_value();

            if lowest.is_some() {
                let lowest = lowest.unwrap();
                if lowest < mut_value {
                    std::mem::swap(lowest, mut_value);
                }
            }

            self.next.as_mut().unwrap().sort();
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<usize> for LinkedList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if let Some(value) = self.get(index) {
            value
        } else {
            panic!("{OUT_OF_BOUND}")
        }
    }
}

impl<T> IndexMut<usize> for LinkedList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index == 0 && self.value.is_some() {
            self.value.as_mut().unwrap()
        } else if self.next.is_some() {
            self.next.as_mut().unwrap().index_mut(index - 1)
        } else {
            panic!("{OUT_OF_BOUND}")
        }
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIterator::new(self)
    }
}

#[cfg(test)]
mod test_linked_list {
    use super::*;

    fn create_and_init_linked_list() -> LinkedList<u8> {
        let mut list = LinkedList::new();
        list.push(0);
        list.push(1);
        list.push(2);
        list.push(3);
        list
    }

    fn create_and_init_unsorted_linked_list() -> LinkedList<u8> {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(3);
        list.push(0);
        list.push(2);
        list
    }

    #[test]
    fn test_len() {
        let list = create_and_init_linked_list();
        let empty = LinkedList::<u8>::new();

        assert_eq!(list.len(), 4);
        assert_eq!(empty.len(), 0);
    }

    #[test]
    fn test_is_empty() {
        let list = create_and_init_linked_list();
        let empty = LinkedList::<u8>::new();

        assert_eq!(list.is_empty(), false);
        assert_eq!(empty.is_empty(), true);
    }

    #[test]
    fn test_push() {
        let list = create_and_init_linked_list();

        assert_eq!(list.into_vec(), vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_pop_first() {
        let mut list = create_and_init_linked_list();

        assert_eq!(list.pop_first(), Some(0));
        assert_eq!(list.pop_first(), Some(1));
        assert_eq!(list.pop_first(), Some(2));
        assert_eq!(list.pop_first(), Some(3));
        assert_eq!(list.pop_first(), None);
        assert_eq!(list.pop_first(), None);
    }

    #[test]
    fn test_pop() {
        let mut list = create_and_init_linked_list();

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(0));
        assert_eq!(list.pop(), None);
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_get() {
        let list = create_and_init_linked_list();

        assert_eq!(list.get(2), Some(&2));
        assert_eq!(list.get(4), None);
    }

    #[test]
    fn test_remove_1() {
        let mut list = create_and_init_linked_list();

        assert_eq!(list.remove(2), 2);
        let expected = vec![0, 1, 3];
        assert_eq!(list.into_iter().collect::<Vec<u8>>(), expected);
    }

    #[test]
    fn test_remove_2() {
        let mut list = create_and_init_linked_list();

        assert_eq!(list.remove(2), 2);
        assert_eq!(list.remove(2), 3);
        assert_eq!(list.remove(0), 0);
        assert_eq!(list.remove(0), 1);
    }

    #[test]
    #[should_panic(expected = "Index out of bound!")]
    fn test_remove_panic_1() {
        let mut list = create_and_init_linked_list();

        assert_eq!(list.remove(2), 2);
        assert_eq!(list.remove(2), 3);
        assert_eq!(list.remove(0), 0);
        assert_eq!(list.remove(0), 1);
        list.remove(0);
    }

    #[test]
    #[should_panic(expected = "Index out of bound!")]
    fn test_remove_panic_2() {
        let mut list = create_and_init_linked_list();

        list.remove(4);
    }

    #[test]
    fn test_insert_first() {
        let mut list = create_and_init_linked_list();

        list.insert(0, 4);

        let expected = vec![4, 0, 1, 2, 3];
        assert_eq!(list.into_iter().collect::<Vec<u8>>(), expected);
    }

    #[test]
    fn test_insert_middle() {
        let mut list = create_and_init_linked_list();

        list.insert(2, 4);

        let expected = vec![0, 1, 4, 2, 3];
        assert_eq!(list.into_iter().collect::<Vec<u8>>(), expected);
    }

    #[test]
    fn test_insert_last() {
        let mut list = create_and_init_linked_list();

        list.insert(4, 4);

        let expected = vec![0, 1, 2, 3, 4];
        assert_eq!(list.into_iter().collect::<Vec<u8>>(), expected);
    }

    #[test]
    fn test_split_off_begin() {
        let mut left = create_and_init_linked_list();
        let right = left.split_off(0);

        let expected_left = Vec::new();
        let expected_right = vec![0, 1, 2, 3];

        assert_eq!(left.into_iter().collect::<Vec<u8>>(), expected_left);
        assert_eq!(right.into_iter().collect::<Vec<u8>>(), expected_right);
    }

    #[test]
    fn test_split_off_middle() {
        let mut left = create_and_init_linked_list();
        let right = left.split_off(2);

        let expected_left = vec![0, 1];
        let expected_right = vec![2, 3];

        assert_eq!(left.into_iter().collect::<Vec<u8>>(), expected_left);
        assert_eq!(right.into_iter().collect::<Vec<u8>>(), expected_right);
    }

    #[test]
    fn test_split_off_last() {
        let mut left = create_and_init_linked_list();
        let right = left.split_off(4);

        let expected_left = vec![0, 1, 2, 3];
        let expected_right = Vec::<u8>::new();

        assert_eq!(left.into_iter().collect::<Vec<u8>>(), expected_left);
        assert_eq!(right.into_iter().collect::<Vec<u8>>(), expected_right);
    }

    #[test]
    fn test_split_off_empty() {
        let mut left = LinkedList::<u8>::new();
        let right = left.split_off(0);

        let expected_left = Vec::<u8>::new();
        let expected_right = Vec::<u8>::new();

        assert_eq!(left.into_iter().collect::<Vec<u8>>(), expected_left);
        assert_eq!(right.into_iter().collect::<Vec<u8>>(), expected_right);
    }

    #[test]
    fn test_get_lowest_ref_mut_value() {
        let mut list = create_and_init_unsorted_linked_list();
        let mut empty = LinkedList::<u8>::new();

        assert_eq!(list.get_lowest_ref_mut_value(), Some(&mut 0));
        assert_eq!(empty.get_lowest_ref_mut_value(), None);
    }

    #[test]
    fn test_is_sorted() {
        let empty = LinkedList::<u8>::new();
        let sorted = create_and_init_linked_list();
        let unsorted = create_and_init_unsorted_linked_list();

        assert_eq!(empty.is_sorted(), true);
        assert_eq!(sorted.is_sorted(), true);
        assert_eq!(unsorted.is_sorted(), false);
    }

    #[test]
    fn test_sort() {
        let mut empty = LinkedList::<u8>::new();
        let mut sorted = create_and_init_linked_list();
        let mut unsorted = create_and_init_unsorted_linked_list();

        empty.sort();
        sorted.sort();
        unsorted.sort();

        assert_eq!(empty.into_iter().collect::<Vec<u8>>(), Vec::new());
        assert_eq!(sorted.into_iter().collect::<Vec<u8>>(), vec![0, 1, 2, 3]);
        assert_eq!(unsorted.into_iter().collect::<Vec<u8>>(), vec![0, 1, 2, 3]);
    }

    #[test]
    #[should_panic(expected = "Index out of bound!")]
    fn test_index() {
        let list = create_and_init_linked_list();

        let panic = 0;

        assert_eq!(list[0], 0);
        assert_eq!(list[3], 3);
        assert_eq!(list[4], panic);
    }

    #[test]
    fn test_index_mut() {
        let mut list = create_and_init_linked_list();

        list[0] = 4;

        assert_eq!(list[0], 4);
    }
}

mod bench_linked_list {}
