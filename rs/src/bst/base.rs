use std::fmt::Debug;
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

pub type Link<Node> = Rc<Node>;

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

pub fn get_extreme<Node: BstNode>(mut node: Link<Node>, direction: Direction) -> Link<Node> {
    loop {
        match node.clone().get_child(direction) {
            Some(child) => {
                node = child;
            }
            None => return node,
        }
    }
}

pub fn next_inorder<Node: BstNode>(node: Link<Node>) -> Option<Link<Node>> {
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
