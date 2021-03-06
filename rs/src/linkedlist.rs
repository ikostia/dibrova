use std::boxed::Box;
use std::mem::replace;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, data: T) {
        let new_head = Some(Box::new(Node {
            data: data,
            next: self.head.take(),
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

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|ref boxed_node| &boxed_node.data)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|boxed_node| &**boxed_node),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_mut().map(|boxed_node| &mut **boxed_node),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = replace(&mut self.head, None);
        while let Some(mut boxed_node) = head {
            head = replace(&mut boxed_node.next, None);
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|boxed_node| &mut **boxed_node);
            &mut node.data
        })
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|boxed_node| &**boxed_node);
            &node.data
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

        let mut list = List::new();
        list.push("hello".to_string());
        list.push("world".to_string());
        assert!(list.pop() == Some(String::from("world")));
        assert!(list.pop() == Some(String::from("hello")));

        let a = Box::new("hello");
        let b = Box::new("world");
        let mut list = List::new();
        list.push(&a);
        list.push(&b);
        assert!(list.pop() == Some(&b));
        assert!(list.pop() == Some(&a));
    }

    #[test]
    fn test_linked_list_peek() {
        let mut list = List::new();
        list.push("hello");
        assert!(list.peek() == Some(&"hello"));
        list.push("world");
        assert!(list.peek() == Some(&"world"));
        list.pop();
        assert!(list.peek() == Some(&"hello"));
    }

    #[test]
    fn test_into_iter() {
        let mut list = List::new();
        list.push(String::from("1"));
        list.push(String::from("0"));
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(String::from("0")));
        assert_eq!(iter.next(), Some(String::from("1")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter() {
        let mut list = List::new();
        let one = 1;
        let zero = 0;
        list.push(one);
        list.push(zero);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn test_iter_mut() {
        let mut list = {
            let mut list = List::new();
            list.push(1);
            list.push(0);
            list
        };
        {
            let mut iter = list.iter_mut();
            let zero = iter.next().expect("expected Some element");
            assert_eq!(zero, &mut 0);
            *zero = 7;
        }
        {
            let mut iter = list.iter_mut();
            let seven = iter.next().expect("expected Some element");
            assert_eq!(seven, &mut 7);
            let one = iter.next().expect("expected Some element");
            assert_eq!(one, &mut 1);
        }
    }
}
