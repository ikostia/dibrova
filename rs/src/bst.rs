use rand::{thread_rng, Rng, ThreadRng};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::{self, Debug};
use std::rc::Rc;

/// A direction of a child relative to a parent
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

    /// Consumes the node, returning contained value
    fn into_value(self) -> Self::Value;

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
                None => false,
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

    /// Remove links to and from other nodes
    fn extract(
        &self,
    ) -> (
        Option<(Direction, Link<Self>)>,
        Option<Link<Self>>,
        Option<Link<Self>>,
    ) {
        let maybe_parent = self.get_parent();
        let left_child = self.get_child(Direction::Left);
        let right_child = self.get_child(Direction::Right);

        let direction_and_parent = maybe_parent.and_then(|parent| {
            self.set_parent(None);
            parent
                .clone()
                .get_direction_of_value(self.as_value())
                .map(move |direction| {
                    parent.set_child(direction, None);
                    (direction, parent)
                })
        });

        if let Some(child) = left_child.as_ref() {
            child.set_parent(None);
            self.set_child(Direction::Left, None);
        }

        if let Some(child) = right_child.as_ref() {
            child.set_parent(None);
            self.set_child(Direction::Right, None);
        }

        (direction_and_parent, left_child, right_child)
    }
}

pub struct SimpleBstNode<Value: PartialEq + PartialOrd> {
    value: Value,
    left_child: RefCell<Option<Link<Self>>>,
    right_child: RefCell<Option<Link<Self>>>,
    parent: RefCell<Option<Link<Self>>>,
}

impl<Value: fmt::Debug + PartialEq + PartialOrd> Debug for SimpleBstNode<Value> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parent_fmt = if let Some(n) = self.parent.borrow().as_ref() {
            format!("Some({:?})", n.as_value())
        } else {
            format!("None")
        };

        let left_fmt = if let Some(n) = self.left_child.borrow().as_ref() {
            format!("Some({:?})", n.as_value())
        } else {
            format!("None")
        };

        let right_fmt = if let Some(n) = self.right_child.borrow().as_ref() {
            format!("Some({:?})", n.as_value())
        } else {
            format!("None")
        };

        write!(
            f,
            "SimpleBstNode {{ {:?} , l: {}, r: {}, p: {}}}",
            self.value, left_fmt, right_fmt, parent_fmt
        )
    }
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

    fn into_value(self) -> Self::Value {
        // Consuming a node, which has pointers to other nodes is
        // a programming error
        assert!(
            self.left_child.borrow().is_none(),
            "Left child is not None before node consumption"
        );
        assert!(
            self.right_child.borrow().is_none(),
            "Right child is not None before node consumption"
        );
        assert!(
            self.parent.borrow().is_none(),
            "Parent is not None before node consumption"
        );
        self.value
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

    fn delete(
        &mut self,
        value: &<Self::Node as BstNode>::Value,
    ) -> Option<<Self::Node as BstNode>::Value>;

    fn find(&self, value: &<Self::Node as BstNode>::Value) -> Option<Link<Self::Node>> {
        let mut maybe_node = self.get_root();
        loop {
            match maybe_node.take() {
                None => return None,
                Some(node) => match node.get_direction_of_value(value) {
                    None => return Some(node),
                    Some(dir) => {
                        maybe_node = node.get_child(dir);
                    }
                },
            }
        }
    }

    fn iter(&self) -> BstIterator<Self> {
        let current = self
            .get_root()
            .map(|root| get_extreme(root, Direction::Left));
        BstIterator {
            _tree: self,
            current,
        }
    }
}

fn get_extreme<Node: BstNode>(mut node: Link<Node>, direction: Direction) -> Link<Node> {
    loop {
        match node.clone().get_child(direction) {
            Some(child) => {
                node = child;
            }
            None => return node,
        }
    }
}

fn next_inorder<Node: BstNode>(node: Link<Node>) -> Option<Link<Node>> {
    node.clone()
        .get_child(Direction::Right)
        .map(|right_subtree_root| get_extreme(right_subtree_root, Direction::Left))
        .or_else(|| {
            let mut node = node;
            while node.is_child(Direction::Right) {
                node = node.get_parent().unwrap();
            }

            if node.is_child(Direction::Left) {
                node.get_parent()
            } else {
                None
            }
        })
}

pub struct SimpleBst<Node: BstNode> {
    root: Option<Link<Node>>,
    rng: RefCell<ThreadRng>,
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

    fn delete(
        &mut self,
        value: &<Self::Node as BstNode>::Value,
    ) -> Option<<Self::Node as BstNode>::Value> {
        match self.find(value) {
            None => None,
            Some(node) => {
                let _ = self.delete_subtree_root(node.clone());
                // Here it is explicitly expected that this is the last pointer to the node
                // Therefore it is very important that the concept of the `Link` is never exposed
                // in the BST API
                let deleted_node = match Rc::try_unwrap(node) {
                    Ok(deleted_node) => deleted_node,
                    Err(_) => {
                        panic!("Freshly deleted node link expected to only have one reference left")
                    }
                };
                Some(deleted_node.into_value())
            }
        }
    }
}

impl<Value: PartialEq + PartialOrd> SimpleBst<SimpleBstNode<Value>> {
    pub fn new() -> Self {
        Self {
            root: None,
            rng: RefCell::new(thread_rng()),
        }
    }

    fn get_random_direction(&self) -> Direction {
        let rn: u8 = self.rng.borrow_mut().gen();
        if rn % 2 == 0 {
            Direction::Left
        } else {
            Direction::Right
        }
    }

    // Given a subtree root, delete the node from a subtree and return
    // a new subtree root, if such exists
    fn delete_subtree_root(
        &mut self,
        subtree_root: Link<SimpleBstNode<Value>>,
    ) -> Option<Link<SimpleBstNode<Value>>> {
        let (maybe_direction_and_parent, maybe_left_child, maybe_right_child) =
            subtree_root.extract();
        let replacement = match (
            maybe_direction_and_parent,
            maybe_left_child,
            maybe_right_child,
        ) {
            (None, None, None) => None,
            (None, Some(child), None) | (None, None, Some(child)) => Some(child),
            (Some((_direction, _parent)), None, None) => None,
            (Some((direction, parent)), Some(child), None)
            | (Some((direction, parent)), None, Some(child)) => {
                parent.set_child(direction, Some(child.clone()));
                child.set_parent(Some(parent));
                Some(child)
            }
            (maybe_direction_and_parent, Some(left_child), Some(right_child)) => {
                let replacement = match self.get_random_direction() {
                    Direction::Left => {
                        if left_child.get_child(Direction::Right).is_some() {
                            // `left_child` has a right subtree, so rightmost is not equal to left_child
                            let rightmost_in_left_subtree =
                                get_extreme(left_child.clone(), Direction::Right);
                            let _ = rightmost_in_left_subtree.extract();
                            rightmost_in_left_subtree
                                .set_child(Direction::Left, Some(left_child.clone()));
                            left_child.set_parent(Some(rightmost_in_left_subtree.clone()));
                            rightmost_in_left_subtree
                                .set_child(Direction::Right, Some(right_child.clone()));
                            right_child.set_parent(Some(rightmost_in_left_subtree.clone()));
                            rightmost_in_left_subtree
                        } else {
                            // `left_child` has no right subtree, so we can just lift it 1 level
                            left_child.set_child(Direction::Right, Some(right_child.clone()));
                            right_child.set_parent(Some(left_child.clone()));
                            left_child
                        }
                    }
                    Direction::Right => {
                        if right_child.get_child(Direction::Left).is_some() {
                            // `right_child` has a left subtree, so leftmost is not equal to right_child
                            let leftmost_in_right_subtree =
                                get_extreme(right_child.clone(), Direction::Left);
                            let _ = leftmost_in_right_subtree.extract();
                            leftmost_in_right_subtree
                                .set_child(Direction::Right, Some(right_child.clone()));
                            right_child.set_parent(Some(leftmost_in_right_subtree.clone()));
                            leftmost_in_right_subtree
                                .set_child(Direction::Left, Some(left_child.clone()));
                            left_child.set_parent(Some(leftmost_in_right_subtree.clone()));
                            leftmost_in_right_subtree
                        } else {
                            // `right_child` has no left subtree, so we can just lift it 1 level
                            right_child.set_child(Direction::Left, Some(left_child.clone()));
                            left_child.set_parent(Some(right_child.clone()));
                            right_child
                        }
                    }
                };

                if let Some((direction, parent)) = maybe_direction_and_parent {
                    replacement.set_parent(Some(parent.clone()));
                    parent.set_child(direction, Some(replacement.clone()));
                }
                Some(replacement)
            }
        };
        if Some(subtree_root.clone()) == self.root {
            // `subtree_root` was also a main tree root, needs to be replaced
            self.root = replacement.clone();
        }
        replacement
    }
}

pub struct BstIterator<'a, Tree: Bst + ?Sized + 'a> {
    _tree: &'a Tree,
    current: Option<Link<<Tree as Bst>::Node>>,
}

impl<'a, Tree: Bst + ?Sized + 'a> Iterator for BstIterator<'a, Tree> {
    type Item = &'a <<Tree as Bst>::Node as BstNode>::Value;

    fn next(&mut self) -> Option<Self::Item> {
        let to_return = self.current.clone().map(|current| {
            let raw_current: *const <Tree as Bst>::Node = Link::into_raw(current);
            let lifetimed_ref_node: &'a <Tree as Bst>::Node = unsafe { &*raw_current };
            // Note: creating this `Rc` from `raw_current` is necessary, so that `drop`
            // is executed and the refcount is properly decreased for this link
            let _rc_to_be_dropped = unsafe { Link::from_raw(raw_current) };
            lifetimed_ref_node.as_value()
        });
        self.current = self
            .current
            .clone()
            .and_then(|current| next_inorder(current));
        to_return
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

    fn test_simple_node_creation_gen<V: Debug + Clone + Debug + PartialEq + PartialOrd>(
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

    fn test_simple_node_relationships_gen<V: Debug + Clone + Debug + PartialEq + PartialOrd>(
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

    fn test_simple_node_ordering_gen<V: Debug + Clone + Debug + PartialEq + PartialOrd>(
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

    fn empty_simple_bst<V: Clone + Debug + PartialEq + PartialOrd>() -> SimpleBst<SimpleBstNode<V>>
    {
        SimpleBst::<SimpleBstNode<V>>::new()
    }

    fn test_simple_bst_creation_gen<V: Debug + Clone + Debug + PartialEq + PartialOrd>(
        root_value: V,
    ) {
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

    fn test_simple_bst_insertion_gen<V: Debug + Clone + Debug + PartialEq + PartialOrd>(
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

    fn test_get_extreme_gen<V: Debug + Clone + Debug + PartialEq + PartialOrd>(
        small: V,
        medium: V,
        large: V,
    ) {
        // Let's make sure we're not shooting ourselves in the foot by creating incorrect tests
        assert!(small < medium);
        assert!(large > medium);
        let (root_node, left_node, right_node) =
            get_three_nodes::<SimpleBstNode<V>>(small.clone(), medium.clone(), large.clone());
        assert_eq!(get_extreme(root_node.clone(), Direction::Left), left_node);
        assert_eq!(get_extreme(root_node.clone(), Direction::Right), right_node);
        assert_eq!(get_extreme(left_node.clone(), Direction::Left), left_node);
        assert_eq!(get_extreme(left_node.clone(), Direction::Right), left_node);
        assert_eq!(get_extreme(right_node.clone(), Direction::Left), right_node);
        assert_eq!(
            get_extreme(right_node.clone(), Direction::Right),
            right_node
        );
    }

    #[test]
    fn test_get_extreme() {
        test_get_extreme_gen::<u32>(7, 8, 9);
    }

    fn test_next_inorder_gen<V: Debug + Clone + Debug + PartialEq + PartialOrd>(
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
        let e1node = get_extreme(root, Direction::Left);
        let e2node = next_inorder(e1node.clone()).unwrap();
        let e3node = next_inorder(e2node.clone()).unwrap();
        let e4node = next_inorder(e3node.clone()).unwrap();
        let should_be_none = next_inorder(e4node.clone());

        assert_eq!(e1node.as_value(), &e1);
        assert_eq!(e2node.as_value(), &e2);
        assert_eq!(e3node.as_value(), &e3);
        assert_eq!(e4node.as_value(), &e4);
        assert!(should_be_none.is_none());
    }

    #[test]
    fn test_next_inroder() {
        test_next_inorder_gen::<u32>(1, 2, 3, 4);
    }

    fn test_iter_gen<V: Debug + Clone + Debug + PartialEq + PartialOrd>(
        e1: V,
        e2: V,
        e3: V,
        e4: V,
    ) {
        // Let's make sure we're not shooting ourselves in the foot by creating incorrect tests
        assert!((e1 < e2) && (e2 < e3) && (e3 < e4));

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
        let iter = bst.iter();
        let v: Vec<&V> = iter.collect();
        assert_eq!(v, vec![&e1, &e2, &e3, &e4]);

        // TODO: use something like trybuild to verify that dropping
        // `bst`, while using `iter` causes compilation errors
    }

    #[test]
    fn test_iter() {
        test_iter_gen::<u32>(1, 2, 3, 4);
        test_iter_gen::<&u32>(&1, &2, &3, &4);
        // non-static lifetime
        let e1 = 1;
        let e2 = 2;
        let e3 = 3;
        let e4 = 4;
        test_iter_gen::<&u32>(&e1, &e2, &e3, &e4);
        let e1 = String::from("a");
        let e2 = String::from("b");
        let e3 = String::from("c");
        let e4 = String::from("d");
        test_iter_gen::<&str>(&e1, &e2, &e3, &e4);
    }

    fn test_find_gen<V: Debug + Clone + Debug + PartialEq + PartialOrd>(
        e1: V,
        e2: V,
        e3: V,
        e4: V,
        e5: V,
        e6: V,
    ) {
        // Let's make sure we're not shooting ourselves in the foot by creating incorrect tests
        assert!((e1 < e2) && (e2 < e3) && (e3 < e4) && (e4 < e5) && (e5 < e6));
        //   e3
        //  /  \
        // e1   e5
        //     /
        //    e4
        let mut bst = empty_simple_bst::<V>();
        bst.insert(e3.clone());
        bst.insert(e5.clone());
        bst.insert(e1.clone());
        bst.insert(e4.clone());
        assert_eq!(bst.find(&e3).unwrap().as_value(), &e3);
        assert_eq!(bst.find(&e5).unwrap().as_value(), &e5);
        assert_eq!(bst.find(&e1).unwrap().as_value(), &e1);
        assert_eq!(bst.find(&e4).unwrap().as_value(), &e4);
        assert!(bst.find(&e2).is_none());
        assert!(bst.find(&e6).is_none());
    }

    #[test]
    fn test_find() {
        test_find_gen::<u32>(1, 2, 3, 4, 5, 6);
        test_find_gen::<&str>("1", "2", "3", "4", "5", "6");
    }

    fn test_extract_gen<V: Debug + Clone + Debug + PartialEq + PartialOrd>(
        e1: V,
        e2: V,
        e3: V,
        e4: V,
        e5: V,
        e6: V,
    ) {
        // Let's make sure we're not shooting ourselves in the foot by creating incorrect tests
        assert!((e1 < e2) && (e2 < e3) && (e3 < e4) && (e4 < e5) && (e5 < e6));

        //     e3
        //   /   \
        // e1     e5
        //  \    /  \
        //  e2  e4  e6
        let mut bst = empty_simple_bst::<V>();
        bst.insert(e3.clone());
        bst.insert(e1.clone());
        bst.insert(e5.clone());
        bst.insert(e2.clone());
        bst.insert(e4.clone());
        bst.insert(e6.clone());

        let e3node = bst.get_root().unwrap();
        assert_eq!(e3node.as_value(), &e3);
        let e1node = e3node.get_child(Direction::Left).unwrap();
        assert_eq!(e1node.as_value(), &e1);
        let e2node = e1node.get_child(Direction::Right).unwrap();
        assert_eq!(e2node.as_value(), &e2);
        let e5node = e3node.get_child(Direction::Right).unwrap();
        assert_eq!(e5node.as_value(), &e5);
        let e4node = e5node.get_child(Direction::Left).unwrap();
        assert_eq!(e4node.as_value(), &e4);
        let e6node = e5node.get_child(Direction::Right).unwrap();
        assert_eq!(e6node.as_value(), &e6);

        // Delete node without children
        let (maybe_direction_and_parent, maybe_left_child, maybe_right_child) = e2node.extract();
        let (direction, parent) = maybe_direction_and_parent.unwrap();
        assert_eq!(direction, Direction::Right);
        assert_eq!(parent, e1node);
        assert_eq!(maybe_left_child, None);
        assert_eq!(maybe_right_child, None);

        // Delete node with two children
        let (maybe_direction_and_parent, maybe_left_child, maybe_right_child) = e5node.extract();
        let (direction, parent) = maybe_direction_and_parent.unwrap();
        assert_eq!(direction, Direction::Right);
        assert_eq!(parent, e3node);
        assert_eq!(maybe_left_child, Some(e4node));
        assert_eq!(maybe_right_child, Some(e6node));

        // Delete root
        let (maybe_direction_and_parent, maybe_left_child, maybe_right_child) = e3node.extract();
        assert_eq!(maybe_direction_and_parent, None);
        assert_eq!(maybe_left_child, Some(e1node));
        assert_eq!(maybe_right_child, None);

        //     e3
        //   /   \
        // e1     e5
        //  \    /  \
        //  e2  e4  e6
        let mut bst = empty_simple_bst::<V>();
        bst.insert(e3.clone());
        bst.insert(e1.clone());
        bst.insert(e5.clone());
        bst.insert(e2.clone());
        bst.insert(e4.clone());
        bst.insert(e6.clone());

        let e3node = bst.get_root().unwrap();
        assert_eq!(e3node.as_value(), &e3);
        let e1node = e3node.get_child(Direction::Left).unwrap();
        assert_eq!(e1node.as_value(), &e1);
        let e2node = e1node.get_child(Direction::Right).unwrap();
        assert_eq!(e2node.as_value(), &e2);
        let e5node = e3node.get_child(Direction::Right).unwrap();
        assert_eq!(e5node.as_value(), &e5);
        let e4node = e5node.get_child(Direction::Left).unwrap();
        assert_eq!(e4node.as_value(), &e4);
        let e6node = e5node.get_child(Direction::Right).unwrap();
        assert_eq!(e6node.as_value(), &e6);

        let (maybe_direction_and_parent, maybe_left_child, maybe_right_child) = e6node.extract();
        let (direction, parent) = maybe_direction_and_parent.unwrap();
        assert_eq!(direction, Direction::Right);
        assert_eq!(parent, e5node);
        assert_eq!(maybe_left_child, None);
        assert_eq!(maybe_right_child, None);
    }

    #[test]
    fn test_extract() {
        test_extract_gen::<u32>(1, 2, 3, 4, 5, 6);
    }

    fn test_delete_gen<V: Debug + Clone + Debug + PartialEq + PartialOrd>(
        e1: V,
        e2: V,
        e3: V,
        e4: V,
        e5: V,
        e6: V,
    ) {
        // Let's make sure we're not shooting ourselves in the foot by creating incorrect tests
        assert!((e1 < e2) && (e2 < e3) && (e3 < e4) && (e4 < e5) && (e5 < e6));
        //     e3
        //   /   \
        // e1     e5
        //  \    /  \
        //  e2  e4  e6
        let mut bst = empty_simple_bst::<V>();
        bst.insert(e3.clone());
        bst.insert(e1.clone());
        bst.insert(e5.clone());
        bst.insert(e2.clone());
        bst.insert(e4.clone());
        bst.insert(e6.clone());

        assert_eq!(bst.delete(&e5), Some(e5));
        {
            let v: Vec<&V> = bst.iter().collect();
            assert_eq!(v, [&e1, &e2, &e3, &e4, &e6]);
        }

        assert_eq!(bst.delete(&e3), Some(e3));
        {
            let v: Vec<&V> = bst.iter().collect();
            assert_eq!(v, [&e1, &e2, &e4, &e6]);
        }
        assert_eq!(bst.delete(&e4), Some(e4));
        {
            let v: Vec<&V> = bst.iter().collect();
            assert_eq!(v, [&e1, &e2, &e6]);
        }
        assert_eq!(bst.delete(&e2), Some(e2));
        {
            let v: Vec<&V> = bst.iter().collect();
            assert_eq!(v, [&e1, &e6]);
        }
        assert_eq!(bst.delete(&e6), Some(e6));
        {
            let v: Vec<&V> = bst.iter().collect();
            assert_eq!(v, [&e1]);
        }
        assert_eq!(bst.delete(&e1), Some(e1.clone()));
        {
            let v: Vec<&V> = bst.iter().collect();
            assert!(v.is_empty());
        }
        assert_eq!(bst.delete(&e1), None);
    }

    #[test]
    fn test_delete() {
        test_delete_gen::<u32>(1, 2, 3, 4, 5, 6);
    }

}
