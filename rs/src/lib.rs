trait IntDSU {
    /// Create a new disjoint set union structure
    fn new(n: usize) -> Self;

    /// Find the leader of the set to which i belongs
    fn find_leader(&mut self, i: usize) -> usize;

    /// Make l the leader of the set to which i belongs
    /// (l is expected to not belong to i's set at first)
    fn set_leader(&mut self, i: usize, l: usize);

    /// Check if two elements belong to the same set
    fn is_same_set(&mut self, i: usize, j: usize) -> bool {
        self.find_leader(i) == self.find_leader(j)
    }

    /// Merge sets containing i and j into a signle set
    fn join(&mut self, i: usize, j: usize) {
        let il = self.find_leader(i);
        let jl = self.find_leader(j);
        self.set_leader(il, jl);
    }
}

#[derive(Debug)]
struct BaseDSU {
    parent: Vec<usize>
}

impl IntDSU for BaseDSU {
    fn new(n: usize) -> Self {
        let mut v = Vec::with_capacity(n);
        for i in 0..n {
            v.push(i)
        }
        BaseDSU { parent: v }
    }

    fn find_leader(&mut self, i: usize) -> usize {
        let mut i = i;
        while i != self.parent[i] {
            i = self.parent[i];
        }
        i
    }

    fn set_leader(&mut self, i: usize, l: usize) {
        self.parent[i] = l;
    }
}

#[derive(Debug)]
struct DsuWithPathCompression {
    parent: Vec<usize>
}

impl IntDSU for DsuWithPathCompression {
    fn new(n: usize) -> Self {
        let mut v = Vec::with_capacity(n);
        for i in 0..n {
            v.push(i)
        }
        DsuWithPathCompression { parent: v }
    }

    fn find_leader(&mut self, i: usize) -> usize {
        let mut i = i;
        let mut path: Vec<usize> = vec![];
        while i != self.parent[i] {
            path.push(i);
            i = self.parent[i];
        }
        for element in path {
            self.parent[element] = i;
        }
        i
    }

    fn set_leader(&mut self, i: usize, l: usize) {
        self.parent[i] = l;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_basic_dsu_operations<T: IntDSU>(dsu: &mut T) {
        assert_eq!(dsu.is_same_set(1, 2), false);
        dsu.join(1, 2);
        assert_eq!(dsu.is_same_set(1, 2), true);
        dsu.join(1, 3);
        assert_eq!(dsu.is_same_set(3, 2), true);
    }

    #[test]
    fn test_basic_operations_for_base_dsu() {
        let mut base_dsu = BaseDSU::new(10);
        test_basic_dsu_operations(&mut base_dsu);
    }

    #[test]
    fn test_basic_operations_for_dsu_with_parent_compression() {
        let mut dsu_with_parent_compression = DsuWithPathCompression::new(10);
        test_basic_dsu_operations(&mut dsu_with_parent_compression);
    }
}