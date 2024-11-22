use crate::List;

const OUT_OF_BOUND: &str = "Index out of bound!";

#[derive(Debug)]
pub struct LinkedList<T> {
    value: Option<T>,
    next: Option<Box<LinkedList<T>>>,
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
        } else {
            if index == 1 {
                self.push(element);
            } else {
                panic!("{}", OUT_OF_BOUND)
            }
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
                panic!("{}", OUT_OF_BOUND)
            }
        } else if self.next.is_some() {
            self.next.as_mut().unwrap().remove(index - 1)
        } else {
            panic!("{}", OUT_OF_BOUND)
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

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self {
            value: None,
            next: None,
        }
    }

    fn is_last(&self) -> bool {
        self.next.is_none()
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
        assert_eq!(list.into_vec(), expected);
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
    #[should_panic]
    fn test_remove_panic_1() {
        let mut list = create_and_init_linked_list();

        assert_eq!(list.remove(2), 2);
        assert_eq!(list.remove(2), 3);
        assert_eq!(list.remove(0), 0);
        assert_eq!(list.remove(0), 1);
        list.remove(0);
    }

    #[test]
    #[should_panic]
    fn test_remove_panic_2() {
        let mut list = create_and_init_linked_list();

        list.remove(4);
    }

    #[test]
    fn test_insert_1() {
        let mut list = create_and_init_linked_list();

        list.insert(0, 4);

        let expected = vec![4, 0, 1, 2, 3];
        assert_eq!(list.into_vec(), expected);
    }

    #[test]
    fn test_insert_2() {
        let mut list = create_and_init_linked_list();

        list.insert(4, 4);

        let expected = vec![0, 1, 2, 3, 4];
        assert_eq!(list.into_vec(), expected);
    }
}
