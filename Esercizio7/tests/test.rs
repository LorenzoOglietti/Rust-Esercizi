#[cfg(test)]
mod tests {
    use Esercizio7::double_linked_list::double_linked_list::DoublyPointedList;

    #[test]
    fn test_push_back() {
        let mut list: DoublyPointedList<i32> = DoublyPointedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_push_front() {
        let mut list: DoublyPointedList<i32> = DoublyPointedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_get() {
        let mut list: DoublyPointedList<i32> = DoublyPointedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.get(0), Some(1));
        assert_eq!(list.get(0), Some(2));
        assert_eq!(list.get(0), Some(3));
        assert_eq!(list.get(0), None);

        let mut list: DoublyPointedList<i32> = DoublyPointedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.get(-1), Some(3));
        assert_eq!(list.get(-1), Some(2));
        assert_eq!(list.get(-1), Some(1));
        assert_eq!(list.get(-1), None);

        let mut list: DoublyPointedList<i32> = DoublyPointedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.get(2), Some(3));

        let mut list: DoublyPointedList<i32> = DoublyPointedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.get(-2), Some(2));
    }
}
