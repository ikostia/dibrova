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

    pub fn push_back(&mut self, data: T) {
        let new_tail = Node::new(data);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            },
            None => {
                self.tail = Some(new_tail.clone());
                self.head = Some(new_tail);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                },
                None => {
                    self.tail = None;
                }
            };
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().data
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail);
                },
                None => {
                    self.head = None;
                }
            };
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().data
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_push_and_pop() {
        let mut l = List::new();
        assert_eq!(l.pop_back(), None);
        assert_eq!(l.pop_front(), None);
        l.push_back(8);
        assert_eq!(l.pop_back(), Some(8));
        assert_eq!(l.pop_back(), None);
        assert_eq!(l.pop_front(), None);
        l.push_front(7);
        assert_eq!(l.pop_back(), Some(7));
        assert_eq!(l.pop_back(), None);
        assert_eq!(l.pop_front(), None);
        l.push_back(1);
        l.push_back(2);
        l.push_back(3);
        assert_eq!(l.pop_front(), Some(1));
        assert_eq!(l.pop_front(), Some(2));
        l.push_front(4);
        l.push_front(5);
        assert_eq!(l.pop_back(), Some(3));
        assert_eq!(l.pop_back(), Some(4));
        assert_eq!(l.pop_back(), Some(5));
        assert_eq!(l.pop_back(), None)
    }
}
