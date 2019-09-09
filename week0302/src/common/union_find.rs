use std::mem::replace;

pub struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
    num_sets: usize
}

impl UnionFind {

    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            parents: (0..n).collect(),
            ranks: (0..n).collect(),
            num_sets: n
        }
    }

    pub fn num_sets(&self) -> usize {
        return self.num_sets;
    }

    pub fn find(&self, mut x: usize) -> usize {
        while x != self.parents[x] {
            x = self.parents[x];
        }
        return x;
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let mut x_root = self.find(x);
        let mut y_root = self.find(y);
        if x_root == y_root {
            return;
        }
        self.num_sets -= 1;
        if self.ranks[x_root] < self.ranks[y_root] {
            x_root = replace(&mut y_root, x_root);
        }
        self.parents[y_root] = x_root;
        if self.ranks[y_root] == self.ranks[x_root] {
            self.ranks[x_root] += 1;
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_union_find() {
        let mut u = super::UnionFind::new(5);
        assert_eq!(5, u.num_sets());
        u.union(2,1);
        u.union(2,3);
        assert_eq!(3, u.num_sets());
        u.union(3,1);
        assert_eq!(3, u.num_sets());
        u.union(4,0);
        assert_eq!(2, u.num_sets());
    }

    #[test]
    fn test_union_find2() {
        let mut u = super::UnionFind::new(8);
        u.union(4,5);
        u.union(0,5);
        u.union(0,2);
        u.union(1,4);
        assert_eq!(4, u.num_sets());
    }
}