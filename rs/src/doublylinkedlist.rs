use std::rc::Rc;
use std::cell::RefCell;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            data: data,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }

    pub fn push_front(&mut self, data: T) {
        let new_head = Node::new(data);
        match self.head.take() {
            Some(old_head) => {
                // Some head exists
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            },
            None => {
                // adding to an empty list
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        };
    }
}