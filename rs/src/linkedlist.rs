use std::boxed::Box;
use std::mem::replace;

pub struct List<T> {
    head: Link<T>
}

pub struct IntoIter<T>(List<T>);

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

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|ref boxed_node| {
            &boxed_node.data
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
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
}
