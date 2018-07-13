use std::hash::Hash;
use std::cmp::Eq;
use std::cell::RefCell;
use std::collections::{HashMap};

/// Generic trait to represent the Disjoint Set Union structure
trait DSU<'a, T: Hash + Eq + 'a> {
    /// Return true if elements i and j are in the same set
    fn is_same_set(&self, i: &'a T, j: &'a T) -> bool;

    /// Merge sets containing elements i and j
    /// Any implementation would need to make sure that
    /// join utilizes the interior mutability
    fn join(&self, i: &'a T, j: &'a T);

    /// Inserts an element into its own set
    fn insert(&mut self, el: &'a T);
}

/// A trait to represent a forest
trait Forest<'a, T: Hash + Eq + 'a> {
    /// Return the parent node of node i in the tree
    /// Returning the same node symbolizes the tree root.
    fn get_parent(&self, i: &'a T) -> &'a T;

    /// Set the parent of node i in the tree to be node p
    /// Any implementation would need to make sure that
    /// set_parent utilizes the interior mutability
    fn set_parent(&self, i: &'a T, p: &'a T);

    /// Find the root of the tree, containing i
    fn find_root(&self, i: &'a T) -> &'a T {
        let mut i = i;
        let mut j = self.get_parent(i);
        while j != i {
            i = j;
            j = self.get_parent(i);
        }
        i
    }

    /// Create a new tree with el as its root
    fn new_tree_from_root(&mut self, el: &'a T);
}

/// Any Forest is also a DSU
impl<'a, T, F> DSU<'a, T> for F
where
    T: Hash + Eq + 'a,
    F: Forest<'a, T>
{
    fn is_same_set(&self, i: &'a T, j: &'a T) -> bool {
        self.find_root(i) == self.find_root(j)
    }

    fn join(&self, i: &'a T, j: &'a T) {
        let il = self.find_root(i);
        let jl = self.find_root(j);
        self.set_parent(il, jl);
    }

    fn insert(&mut self, el: &'a T) {
        self.new_tree_from_root(el);
    }
}

#[derive(Debug)]
pub struct ForestDsu<'a, T: 'a + Hash + Eq>
{
    parents: RefCell<HashMap<&'a T, &'a T>>
}

impl<'a, T: 'a + Hash + Eq> ForestDsu<'a, T>
{
    pub fn new() -> Self {
        Self { parents: RefCell::new(HashMap::new()) }
    }
}

impl<'a, T> Forest<'a, T> for ForestDsu<'a, T>
where
    T: 'a + Hash + Eq
{
    fn new_tree_from_root(&mut self, el: &'a T) {
        (*self.parents.borrow_mut()).insert(el, el);
    }

    fn get_parent(&self, i: &'a T) -> &'a T {
        self.parents.borrow()[i]
    }

    fn set_parent(&self, i: &'a T, p: &'a T) {
        (*self.parents.borrow_mut()).insert(i, p);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_basic_dsu_operations<'a, T, D>(dsu: &mut D, elements: &'a [T]) 
    where
        T: 'a + Hash + Eq,
        D: DSU<'a, T>
    {
        dsu.insert(&elements[0]);
        dsu.insert(&elements[1]);
        dsu.insert(&elements[2]);
        assert_eq!(dsu.is_same_set(&elements[0], &elements[1]), false);
        dsu.join(&elements[0], &elements[1]);
        assert_eq!(dsu.is_same_set(&elements[0], &elements[1]), true);
        dsu.join(&elements[0], &elements[2]);
        assert_eq!(dsu.is_same_set(&elements[2], &elements[1]), true);
    }

    #[test]
    fn test_basic_operations_for_base_dsu<'a>() {
        let mut base_dsu: ForestDsu<'a, usize> = ForestDsu::new();
        test_basic_dsu_operations(&mut base_dsu, &[1, 2, 3]);
    }
}