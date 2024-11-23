use std::ops::Index;
use std::ops::IndexMut;

use crate::List;
use crate::linked_list_iterator::LinkedListIterator;

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
