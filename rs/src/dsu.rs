use std::cell::RefCell;
use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

/// Generic trait to represent the Disjoint Set Union structure
pub trait DSU<'a, T: Hash + Eq + 'a> {
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
pub trait Forest<'a, T: Hash + Eq + 'a> {
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

    /// A function to decide which tree to merge into
    /// which if merging happens. First a new root should be returned,
    /// then a new child
    fn get_merge_direction(&self, i: &'a T, j: &'a T) -> (&'a T, &'a T) {
        (i, j)
    }
}

/// Any Forest is also a DSU
impl<'a, T, F> DSU<'a, T> for F
where
    T: Hash + Eq + 'a,
    F: Forest<'a, T>,
{
    fn is_same_set(&self, i: &'a T, j: &'a T) -> bool {
        self.find_root(i) == self.find_root(j)
    }

    fn join(&self, i: &'a T, j: &'a T) {
        let il = self.find_root(i);
        let jl = self.find_root(j);
        if il != jl {
            let (new_root, new_child) = self.get_merge_direction(il, jl);
            self.set_parent(new_child, new_root);
        }
    }

    fn insert(&mut self, el: &'a T) {
        self.new_tree_from_root(el);
    }
}

/// A naive unoptimized implementation of
/// the forest-based DSU, without path
/// compression or size heuristics
#[derive(Debug)]
pub struct ForestDsu<'a, T: 'a + Hash + Eq> {
    parents: RefCell<HashMap<&'a T, &'a T>>,
}

impl<'a, T: 'a + Hash + Eq> ForestDsu<'a, T> {
    pub fn new() -> Self {
        Self {
            parents: RefCell::new(HashMap::new()),
        }
    }
}

impl<'a, T> Forest<'a, T> for ForestDsu<'a, T>
where
    T: 'a + Hash + Eq,
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

/// A forest-based DSU implementation with
/// path compression and size heuristics
#[derive(Debug)]
pub struct OptimizedForestDsu<'a, T: 'a + Hash + Eq> {
    parents: RefCell<HashMap<&'a T, &'a T>>,
    sizes: RefCell<HashMap<&'a T, usize>>,
}

impl<'a, T: 'a + Hash + Eq> OptimizedForestDsu<'a, T> {
    pub fn new() -> Self {
        Self {
            parents: RefCell::new(HashMap::new()),
            sizes: RefCell::new(HashMap::new()),
        }
    }
}

impl<'a, T> Forest<'a, T> for OptimizedForestDsu<'a, T>
where
    T: 'a + Hash + Eq,
{
    fn new_tree_from_root(&mut self, el: &'a T) {
        (*self.parents.borrow_mut()).insert(el, el);
        (*self.sizes.borrow_mut()).insert(el, 1);
    }

    fn get_parent(&self, i: &'a T) -> &'a T {
        self.parents.borrow()[i]
    }

    fn set_parent(&self, i: &'a T, p: &'a T) {
        (*self.parents.borrow_mut()).insert(i, p);
        let sz = self.sizes.borrow()[i] + self.sizes.borrow()[p];
        (*self.sizes.borrow_mut()).insert(i, sz);
    }

    /// Find the root of the tree, containing i
    fn find_root(&self, i: &'a T) -> &'a T {
        let mut i = i;
        let mut j = self.get_parent(i);
        let mut path: Vec<&'a T> = vec![];
        while j != i {
            path.push(i);
            i = j;
            j = self.get_parent(i);
        }
        // perform path compression
        for el in path {
            self.set_parent(el, i);
        }
        i
    }

    fn get_merge_direction(&self, i: &'a T, j: &'a T) -> (&'a T, &'a T) {
        if self.sizes.borrow()[i] > self.sizes.borrow()[j] {
            (i, j)
        } else {
            (j, i)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_basic_dsu_operations<'a, T, D>(dsu: &mut D, elements: &'a [T])
    where
        T: 'a + Hash + Eq,
        D: DSU<'a, T>,
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
    fn test_basic_operations_for_naive_dsu<'a>() {
        let mut dsu: ForestDsu<usize> = ForestDsu::new();
        test_basic_dsu_operations(&mut dsu, &[1, 2, 3]);
    }

    #[test]
    fn test_basic_operations_for_optimized_dsu<'a>() {
        let mut dsu: OptimizedForestDsu<usize> = OptimizedForestDsu::new();
        test_basic_dsu_operations(&mut dsu, &[1, 2, 3]);
    }

}
