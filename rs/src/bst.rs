use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

/// A direction of a child relative to a parent
#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

/// Gets the other direction
pub fn flip_direction(d: Direction) -> Direction {
    match d {
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
    }
}

type Link<Node> = Rc<Node>;

/// Represents the basic structure of a BST Node
pub trait BstNode: PartialEq {
    type Value: PartialEq + PartialOrd;

    fn new(value: Self::Value) -> Self;

    /// Returns the reference to the stored value
    fn as_value(&self) -> &Self::Value;

    /// Returns an `Option` with a reference to a child if present
    fn get_child(&self, direction: Direction) -> Option<Link<Self>>;

    /// Returns an `Option` with a reference to a parent if present
    fn get_parent(&self) -> Option<Link<Self>>;

    /// Sets `self`'s child in appropriate direction
    fn set_child(&self, direction: Direction, child: Option<Link<Self>>);

    /// Sets `self`'s parent
    fn set_parent(&self, parent: Option<Link<Self>>);

    /// Checks whether the node is a leaf
    fn is_leaf(&self) -> bool {
        self.get_child(Direction::Left).is_none() && self.get_child(Direction::Right).is_none()
    }

    /// Checks whether the node is a root
    fn is_root(&self) -> bool {
        self.get_parent().is_none()
    }

    /// Checks whether the node is a `direction` child of its parent
    fn is_child(&self, direction: Direction) -> bool {
        match self.get_parent() {
            None => false,
            Some(parent) => match parent.get_child(direction) {
                None => panic!("Inconsistent tree structure: parent of node has empty child link"),
                Some(child) => &*child == self,
            },
        }
    }

    /// Return the direction of descent where `v` can be located
    /// Return value `None` implies the equivalency of `self.as_value()`
    /// and `v`
    fn get_direction_of_value(&self, v: &Self::Value) -> Option<Direction> {
        if v < self.as_value() {
            Some(Direction::Left)
        } else if v > self.as_value() {
            Some(Direction::Right)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct SimpleBstNode<Value: PartialEq + PartialOrd> {
    value: Value,
    left_child: RefCell<Option<Link<Self>>>,
    right_child: RefCell<Option<Link<Self>>>,
    parent: RefCell<Option<Link<Self>>>,
}

impl<Value: PartialEq + PartialOrd> PartialEq<SimpleBstNode<Value>> for SimpleBstNode<Value> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<Value: PartialEq + PartialOrd> PartialOrd for SimpleBstNode<Value> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_value().partial_cmp(other.as_value())
    }
}

impl<Value: PartialEq + PartialOrd> BstNode for SimpleBstNode<Value> {
    type Value = Value;

    fn new(value: Value) -> Self {
        Self {
            value: value,
            left_child: RefCell::new(None),
            right_child: RefCell::new(None),
            parent: RefCell::new(None),
        }
    }

    fn as_value(&self) -> &Self::Value {
        &self.value
    }

    fn get_child(&self, direction: Direction) -> Option<Link<Self>> {
        match direction {
            Direction::Left => self.left_child.borrow().clone(),
            Direction::Right => self.right_child.borrow().clone(),
        }
    }

    fn get_parent(&self) -> Option<Link<Self>> {
        self.parent.borrow().clone()
    }

    fn set_child(&self, direction: Direction, child: Option<Link<Self>>) {
        match direction {
            Direction::Left => *self.left_child.borrow_mut() = child,
            Direction::Right => *self.right_child.borrow_mut() = child,
        }
    }

    fn set_parent(&self, parent: Option<Link<Self>>) {
        *self.parent.borrow_mut() = parent
    }
}

pub trait Bst {
    type Node: BstNode;

    fn get_root(&self) -> Option<Link<Self::Node>>;

    fn insert(&mut self, value: <Self::Node as BstNode>::Value);
}

pub struct SimpleBst<Node: BstNode> {
    root: Option<Link<Node>>,
}

impl<Value: PartialEq + PartialOrd> Bst for SimpleBst<SimpleBstNode<Value>> {
    type Node = SimpleBstNode<Value>;

    fn get_root(&self) -> Option<Link<Self::Node>> {
        self.root.clone()
    }

    fn insert(&mut self, value: <Self::Node as BstNode>::Value) {
        let mut maybe_parent_and_direction: Option<(Link<Self::Node>, Direction)> = None;
        let mut maybe_current_node: Option<Link<Self::Node>> = self.root.clone();
        loop {
            match maybe_current_node.clone() {
                Some(current_node) => {
                    match current_node.get_direction_of_value(&value) {
                        Some(direction) => {
                            maybe_parent_and_direction = Some((current_node.clone(), direction));
                            maybe_current_node = current_node.get_child(direction);
                        }
                        None => break,
                    };
                }
                None => break,
            }
        }

        // if `maybe_current_node.is_some()`, we are inserting a duplicate node
        // let's ignore it for now
        if maybe_current_node.is_some() {
            return;
        }

        let new_node = Link::new(<Self::Node as BstNode>::new(value));
        match maybe_parent_and_direction {
            Some((parent, direction)) => {
                parent.set_child(direction, Some(new_node.clone()));
                new_node.set_parent(Some(parent));
            }
            None => {
                // First node of the tree
                self.root.replace(new_node);
            }
        };
    }
}

impl<Value: PartialEq + PartialOrd> SimpleBst<SimpleBstNode<Value>> {
    pub fn new() -> Self {
        Self { root: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;

    fn get_three_nodes<Node: BstNode>(
        small: Node::Value,
        medium: Node::Value,
        large: Node::Value,
    ) -> (Link<Node>, Link<Node>, Link<Node>) {
        let left_node = Link::new(Node::new(small));
        let right_node = Link::new(Node::new(large));
        let root_node = Link::new(Node::new(medium));
        root_node.set_child(Direction::Left, Some(left_node.clone()));
        root_node.set_child(Direction::Right, Some(right_node.clone()));
        left_node.set_parent(Some(root_node.clone()));
        right_node.set_parent(Some(root_node.clone()));
        (root_node, left_node, right_node)
    }

    fn test_simple_node_creation_gen<V: Debug + Clone + PartialEq + PartialOrd>(
        small: V,
        medium: V,
        large: V,
    ) {
        // Let's make sure we're not shooting ourselves in the foot by creating incorrect tests
        assert!(small < medium);
        assert!(large > medium);
        let (root_node, left_node, right_node) =
            get_three_nodes::<SimpleBstNode<V>>(small.clone(), medium.clone(), large.clone());
        assert_eq!(root_node.as_value(), &medium);
        assert_eq!(left_node.as_value(), &small);
        assert_eq!(right_node.as_value(), &large);
    }

    #[test]
    fn test_simple_node_creation() {
        test_simple_node_creation_gen::<u32>(8, 9, 10);
        test_simple_node_creation_gen::<&u32>(&8, &9, &10);
        test_simple_node_creation_gen::<String>("a".to_string(), "b".to_string(), "c".to_string());
        test_simple_node_creation_gen::<&str>("a", "b", "c");
    }

    fn test_simple_node_relationships_gen<V: Debug + Clone + PartialEq + PartialOrd>(
        small: V,
        medium: V,
        large: V,
    ) {
        // Let's make sure we're not shooting ourselves in the foot by creating incorrect tests
        assert!(small < medium);
        assert!(large > medium);
        let (root_node, left_node, right_node) =
            get_three_nodes::<SimpleBstNode<V>>(small, medium.clone(), large);
        assert_eq!(root_node.get_child(Direction::Left).unwrap(), left_node);
        assert_eq!(root_node.get_child(Direction::Right).unwrap(), right_node);
        assert_eq!(left_node.get_parent().unwrap().as_value(), &medium);
        assert_eq!(right_node.get_parent().unwrap().as_value(), &medium);
        assert_eq!(root_node.is_root(), true);
        assert_eq!(root_node.is_leaf(), false);
        assert_eq!(left_node.is_root(), false);
        assert_eq!(left_node.is_leaf(), true);
        assert_eq!(right_node.is_root(), false);
        assert_eq!(right_node.is_leaf(), true);
    }

    #[test]
    fn test_simple_node_relationships() {
        test_simple_node_relationships_gen::<u32>(8, 9, 10);
        test_simple_node_relationships_gen::<String>(
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
        );
        test_simple_node_relationships_gen::<&u32>(&8, &9, &10);
        test_simple_node_relationships_gen::<&str>("a", "b", "c");
    }

    fn test_simple_node_ordering_gen<V: Debug + Clone + PartialEq + PartialOrd>(
        small: V,
        medium: V,
        large: V,
    ) {
        // Let's make sure we're not shooting ourselves in the foot by creating incorrect tests
        assert!(small < medium);
        assert!(large > medium);

        let (root_node, left_node, right_node) =
            get_three_nodes::<SimpleBstNode<V>>(small, medium, large);
        assert!(left_node < root_node);
        assert!(left_node < right_node);
        assert!(right_node > root_node);
        assert!(right_node > left_node);
        assert!(root_node > left_node);
        assert!(root_node < right_node);
    }

    #[test]
    fn test_simple_node_ordering() {
        test_simple_node_ordering_gen::<u32>(8, 9, 10);
        test_simple_node_ordering_gen::<&u32>(&8, &9, &10);
        test_simple_node_ordering_gen::<&str>("a", "b", "c");
        test_simple_node_ordering_gen::<String>("a".to_string(), "b".to_string(), "c".to_string());
    }

    fn empty_simple_bst<V: PartialEq + PartialOrd>() -> SimpleBst<SimpleBstNode<V>> {
        SimpleBst::<SimpleBstNode<V>>::new()
    }

    fn test_simple_bst_creation_gen<V: Debug + Clone + PartialEq + PartialOrd>(root_value: V) {
        let mut bst = empty_simple_bst::<V>();
        assert!(bst.get_root().is_none());
        bst.insert(root_value.clone());
        assert_eq!(bst.get_root().unwrap().as_value(), &root_value);
    }

    #[test]
    fn test_simple_bst_creation() {
        test_simple_bst_creation_gen::<u32>(1);
        test_simple_bst_creation_gen::<&u32>(&1);
        test_simple_bst_creation_gen::<String>("hello".to_string());
        test_simple_bst_creation_gen::<&str>("hello");
    }

    fn test_simple_bst_insertion_gen<V: Debug + Clone + PartialEq + PartialOrd>(
        e1: V,
        e2: V,
        e3: V,
        e4: V,
    ) {
        // Let's make sure we're not shooting ourselves in the foot by creating incorrect tests
        assert!((e1 < e2) && (e2 < e3) && (e3 < e4));

        // Case 1:
        //   e2
        //  /  \
        // e1   e4
        //     /
        //    e3
        let mut bst = empty_simple_bst::<V>();
        bst.insert(e2.clone());
        bst.insert(e4.clone());
        bst.insert(e1.clone());
        bst.insert(e3.clone());
        let root = bst.get_root().unwrap();
        assert_eq!(root.as_value(), &e2);
        let roots_left_child = root.get_child(Direction::Left).unwrap();
        let roots_right_child = root.get_child(Direction::Right).unwrap();
        assert_eq!(roots_left_child.as_value(), &e1);
        assert_eq!(roots_right_child.as_value(), &e4);
        assert!(roots_left_child.get_child(Direction::Left).is_none());
        assert!(roots_left_child.get_child(Direction::Right).is_none());
        assert!(roots_right_child.get_child(Direction::Right).is_none());
        let roots_grandchild = roots_right_child.get_child(Direction::Left).unwrap();
        assert_eq!(roots_grandchild.as_value(), &e3);

        // Case 2:
        //   e1
        //     \
        //      e2
        //        \
        //         e3
        //          \
        //           e4
        let mut bst = empty_simple_bst::<V>();
        bst.insert(e1.clone());
        bst.insert(e2.clone());
        bst.insert(e3.clone());
        bst.insert(e4.clone());
        let n1 = bst.get_root().unwrap();
        let n2 = n1.get_child(Direction::Right).unwrap();
        let n3 = n2.get_child(Direction::Right).unwrap();
        let n4 = n3.get_child(Direction::Right).unwrap();
        assert!(n4.get_child(Direction::Right).is_none());
        for (node, expected_value) in vec![(n1, e1), (n2, e2), (n3, e3), (n4, e4)] {
            assert_eq!(node.as_value(), &expected_value);
            assert!(node.get_child(Direction::Left).is_none());
        }
    }

    #[test]
    fn test_simple_bst_insertion() {
        test_simple_bst_insertion_gen::<u32>(1, 2, 3, 4);
        test_simple_bst_insertion_gen::<&u32>(&1, &2, &3, &4);
        test_simple_bst_insertion_gen::<String>(
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        );
        test_simple_bst_insertion_gen::<&str>("a", "b", "c", "d");
    }
}
