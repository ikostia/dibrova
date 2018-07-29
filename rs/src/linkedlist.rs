use std::rc::Rc;
use std::cell::RefCell;
use std::boxed::Box;

pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, data: T) {
        let new_head = Some(Box::new(Node {
            data: data,
            next: self.head.take()
        }));

        self.head = new_head;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|boxed_node| {
            let boxed_node = *boxed_node;
            self.head = boxed_node.next;
            boxed_node.data
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list_simple() {
        let mut list = List::new();
        assert!(list.pop() == None);
        list.push(0);
        list.push(1);
        assert!(list.pop() == Some(1));
        assert!(list.pop() == Some(0));
        assert!(list.pop() == None);
    }
}
