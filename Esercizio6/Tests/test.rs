 #[cfg(test)]
    mod tests {
        use Esercizio6::double_linked_list::double_linked_list::DoublyPointedList;

        #[derive(Debug, PartialEq, Copy, Clone)]
        struct Point {
            x: i32,
            y: i32,
        }

        #[test]
        fn test_push_back() {
            let mut list = DoublyPointedList::new();
            assert_eq!(list.len(), 0);

            list.push_back(Point { x: 1, y: 2 });
            assert_eq!(list.len(), 1);
            assert_eq!(list.get(0).unwrap(), Point { x: 1, y: 2 });

            list.push_back(Point { x: 3, y: 4 });
            assert_eq!(list.len(), 2);
            assert_eq!(list.get(1).unwrap(), Point { x: 3, y: 4 });
        }

        #[test]
        fn test_push_front() {
            let mut list = DoublyPointedList::new();
            assert_eq!(list.len(), 0);

            list.push_front(Point { x: 1, y: 2 });
            assert_eq!(list.len(), 1);
            assert_eq!(list.get(0).unwrap(), Point { x: 1, y: 2 });

            list.push_front(Point { x: 3, y: 4 });
            assert_eq!(list.len(), 2);
            assert_eq!(list.get(0).unwrap(), Point { x: 3, y: 4 });
        }

        #[test]
        fn test_pop_back() {
            let mut list = DoublyPointedList::new();
            list.push_back(Point { x: 1, y: 2 });
            list.push_back(Point { x: 3, y: 4 });

            assert_eq!(list.pop_back().unwrap(), Point { x: 3, y: 4 });
            assert_eq!(list.len(), 1);
            assert_eq!(list.pop_back().unwrap(), Point { x: 1, y: 2 });
            assert_eq!(list.len(), 0);
            assert_eq!(list.pop_back(), None);
        }

        #[test]
        fn test_pop_front() {
            let mut list = DoublyPointedList::new();
            list.push_back(Point { x: 1, y: 2 });
            list.push_back(Point { x: 3, y: 4 });

            assert_eq!(list.pop_front().unwrap(), Point { x: 1, y: 2 });
            assert_eq!(list.len(), 1);
            assert_eq!(list.pop_front().unwrap(), Point { x: 3, y: 4 });
            assert_eq!(list.len(), 0);
            assert_eq!(list.pop_front(), None);
        }

        #[test]
        fn test_get() {
            let mut list = DoublyPointedList::new();
            list.push_back(Point { x: 1, y: 2 });
            list.push_back(Point { x: 3, y: 4 });
            list.push_back(Point { x: 5, y: 6 });

            assert_eq!(list.get(0).unwrap(), Point { x: 1, y: 2 });
            assert_eq!(list.get(1).unwrap(), Point { x: 3, y: 4 });
            assert_eq!(list.get(2).unwrap(), Point { x: 5, y: 6 });
            assert_eq!(list.get(-1).unwrap(), Point { x: 5, y: 6 });
            assert_eq!(list.get(-2).unwrap(), Point { x: 3, y: 4 });
            assert_eq!(list.get(-3).unwrap(), Point { x: 1, y: 2 });
            assert_eq!(list.get(3), None);
        }
    }