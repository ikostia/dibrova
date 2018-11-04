use std::mem;
use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: ptr::null_mut() }
    }

    pub fn push_back(&mut self, data: T) {
        let mut new_tail = Box::new(Node {data: data, next: None});
        let new_tail_ptr: *mut _ = &mut *new_tail;

        if self.tail == ptr::null_mut() {
            self.tail = new_tail_ptr;
            self.head = Some(new_tail);
        } else {
            unsafe { (*self.tail).next = Some(new_tail) };
            self.tail = new_tail_ptr;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|boxed_head| {
            let head = *boxed_head;
            let Node {data: head_data, next: head_next} = head;
            if let Some(new_head) = head_next {
                self.head = Some(new_head);
            } else {
                self.head = None;
                self.tail = ptr::null_mut();
            }

            head_data
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let list = List::<u32>::new();
        assert!(list.head.is_none());
        assert_eq!(list.tail, ptr::null_mut());
    }

    #[test]
    fn test_push_pop() {
        let mut list = List::<u32>::new();
        assert_eq!(list.pop_front(), None);
        list.push_back(0);
        assert_eq!(list.pop_front(), Some(0));
        assert_eq!(list.pop_front(), None);
        list.push_back(0);
        list.push_back(1);
        assert_eq!(list.pop_front(), Some(0));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
}