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

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>
}

pub struct IntoIter<T>(List<T>);

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>
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

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|boxed_head_reference| &**boxed_head_reference)
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_mut().map(|boxed_head_mut| &mut **boxed_head_mut)
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {};
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node_reference| {
            self.next = node_reference.next.as_ref().map(|boxed_next_reference| &**boxed_next_reference);
            &node_reference.data
        })
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node_mut| {
            self.next = node_mut.next.as_mut().map(|boxed_next_mut| &mut **boxed_next_mut);
            &mut node_mut.data
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

    #[test]
    fn test_iter() {
        let mut list = List::<u32>::new();
        list.push_back(0);
        list.push_back(1);
        list.push_back(2);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_into_iter() {
        let mut list = List::<u32>::new();
        list.push_back(0);
        list.push_back(1);
        list.push_back(2);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_mut() {
        let mut list = List::<u32>::new();
        list.push_back(0);
        list.push_back(1);
        list.push_back(2);
        {
            let mut iter = list.iter_mut();
            assert_eq!(iter.next(), Some(&mut 0));
            assert_eq!(iter.next(), Some(&mut 1));
            assert_eq!(iter.next(), Some(&mut 2));
            assert_eq!(iter.next(), None);
        }
        {
            let mut iter = list.iter_mut();
            *iter.next().unwrap() = 6;
        }
        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 6));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), None);

    }
}