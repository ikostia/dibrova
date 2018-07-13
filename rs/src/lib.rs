use std::hash::Hash;
use std::cmp::PartialEq;

/// Generic trait to represent the Disjoint Set Union structure
trait DSU<T: Hash + PartialEq + Copy> {
    /// Return true if elements i and j are in the same set
    fn is_same_set(&self, i: T, j: T) -> bool;

    /// Merge sets containing elements i and j
    fn join(&mut self, i: T, j: T);
}

/// A trait to represent the DSU, implemented as a tree
trait TreeDsu<T: Hash + PartialEq + Copy> {
    /// Return the parent node of node i in the tree
    fn get_parent(&self, i: T) -> T;

    /// Set the parent of node i in the tree to be node p
    fn set_parent(&mut self, i: T, p: T);

    /// Find the leader of the set, containing i
    fn find_leader<'a>(&'a self, i: T) -> T {
        let mut i = i;
        let mut j = self.get_parent(i);
        while j != i {
            i = j;
            j = self.get_parent(i);
        }
        i
    }
}

impl<T, TD> DSU<T> for TD
where
    T: Hash + PartialEq + Copy,
    TD: TreeDsu<T>
{
    /// Return true if i and j belong to the same set
    fn is_same_set(&self, i: T, j: T) -> bool {
        self.find_leader(i) == self.find_leader(j)
    }

    /// Merge sets containing elements i and j
    fn join(&mut self, i: T, j: T) {
        let il = self.find_leader(i);
        let jl = self.find_leader(j);
        self.set_parent(il, jl);
    }
}

#[derive(Debug)]
pub struct UsizeDSU {
    parent: Vec<usize>
}

impl UsizeDSU {
    pub fn new(n: usize) -> Self {
        let mut v = Vec::with_capacity(n);
        for i in 0..n {
            v.push(i)
        }
        Self { parent: v }
    }
}

impl TreeDsu<usize> for UsizeDSU {
    fn get_parent(&self, i: usize) -> usize {
        self.parent[i]
    }

    fn set_parent(&mut self, i: usize, p: usize) {
        self.parent[i] = p;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_basic_dsu_operations<T: DSU<usize>>(dsu: &mut T) {
        assert_eq!(dsu.is_same_set(1, 2), false);
        dsu.join(1, 2);
        assert_eq!(dsu.is_same_set(1, 2), true);
        dsu.join(1, 3);
        assert_eq!(dsu.is_same_set(3, 2), true);
    }

    #[test]
    fn test_basic_operations_for_base_dsu() {
        let mut base_dsu = UsizeDSU::new(10);
        test_basic_dsu_operations(&mut base_dsu);
    }
}