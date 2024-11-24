use crate::linked_list::LinkedList;

pub struct LinkedListIterator<T> {
    linked_list: LinkedList<T>,
}

impl<T> LinkedListIterator<T> {
    pub fn new(linked_list: LinkedList<T>) -> Self {
        LinkedListIterator { linked_list }
    }
}

impl<T> Iterator for LinkedListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.linked_list.pop_first()
    }
}

#[cfg(test)]
mod test_linked_list_iterator {
    use super::*;
    use crate::List;

    fn create_and_init_linked_list() -> LinkedList<u8> {
        let mut list = LinkedList::new();
        list.push(0);
        list.push(1);
        list.push(2);
        list.push(3);
        list
    }

    #[test]
    fn test_next() {
        let list = create_and_init_linked_list();
        let mut into_iter = list.into_iter();

        assert_eq!(into_iter.next(), Some(0));
        assert_eq!(into_iter.next(), Some(1));
        assert_eq!(into_iter.next(), Some(2));
        assert_eq!(into_iter.next(), Some(3));
        assert_eq!(into_iter.next(), None);
        assert_eq!(into_iter.next(), None);
    }

    #[test]
    fn test_collect_on_empty() {
        let list = LinkedList::<u8>::new();

        assert_eq!(list.into_iter().collect::<Vec<u8>>(), Vec::<u8>::new())
    }
}
