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

    #[test]
    fn push_test() {
        let mut list = LinkedList::empty();

        list.push_start(12);
        list.push_start(15);
    }
}
