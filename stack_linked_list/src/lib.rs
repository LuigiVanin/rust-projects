type Link<T> = Option<Box<Node<T>>>;

// #[derive(Debug)]
struct Node<T> {
    next: Link<T>,
    element: T,
}

pub struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    fn empty() -> LinkedList<T> {
        LinkedList { head: None }
    }

    fn new(data: Vec<T>) -> LinkedList<T> {
        let mut list = LinkedList::empty();
        for i in data {
            list.push_start(i);
        }
        list
    }

    fn push_start(self: &mut Self, element: T) {
        match self.head.take() {
            old_head => {
                let new_head = Some(Box::new(Node {
                    element,
                    next: old_head,
                }));
                self.head = new_head;
            }
        }
    }

    fn pop_start(self: &mut Self) -> Option<T> {
        match self.head.take() {
            Some(node) => {
                self.head = node.next;
                Some(node.element)
            }
            None => None,
        }
    }

    fn get_top(self: &Self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn push_test_1() {
        let mut list = LinkedList::empty();

        list.push_start(12);
        list.push_start(15);
        assert!(list.get_top() == Some(&15));
        assert!(list.get_top() == Some(&15));

        list.push_start(0);

        assert!(list.get_top() == Some(&0));
    }

    #[test]
    fn push_test_2() {
        let mut rng = rand::thread_rng();
        let mut list = LinkedList::<i32>::empty();

        let mut value = rng.gen_range(0..100);
        list.push_start(value);
        assert!(list.get_top() == Some(&value));

        value = rng.gen_range(0..100);
        list.push_start(value);
        assert!(list.get_top() == Some(&value));
    }

    #[test]
    fn pop_test_1() {
        let mut list = LinkedList::empty();
        let mut rng = rand::thread_rng();

        for _ in 0..20 {
            list.push_start(rng.gen_range(0..100));
        }

        for _ in 0..20 {
            assert!(!list.pop_start().is_none())
        }

        assert!(list.pop_start().is_none())
    }

    #[test]
    fn pop_test_2() {
        let mut list = LinkedList::empty();
        let mut rng = rand::thread_rng();

        for _ in 0..20 {
            list.push_start(rng.gen_range(0..100));
        }

        for _ in 0..20 {
            assert!(!list.pop_start().is_none())
        }

        assert!(list.pop_start().is_none())
    }

    #[test]
    fn new_test_1() {
        let mut list = LinkedList::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        for i in (1..11).rev() {
            assert!(list.pop_start() == Some(i));
        }
    }
}
