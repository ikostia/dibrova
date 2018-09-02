use std::rc::Rc;

pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn append(&self, data: T) -> List<T> {
        List { head: Some(Rc::new(Node {
            data: data,
            next: self.head.clone(),
        }))}
    }

    pub fn tail(&self) -> List<T> {
        List { head: self.head.as_ref().and_then(|head_node| head_node.next.clone()) }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| { &node.data })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_basics() {
        let list = List::new().append(0).append(1);
        assert_eq!(list.head(), Some(&1));
        assert_eq!(list.tail().head(), Some(&0));
        assert_eq!(list.tail().tail().head(), None);
    }
}