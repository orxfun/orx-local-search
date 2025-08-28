use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tour(Vec<usize>);

impl Tour {
    pub fn in_standard_order(num_cities: usize) -> Self {
        Self((0..num_cities).collect())
    }

    pub fn get(&self, position: usize) -> Option<usize> {
        self.0.get(position).copied()
    }

    pub fn example_solution() -> Self {
        Self::in_standard_order(6)
    }
}

impl IntoIterator for Tour {
    type Item = usize;

    type IntoIter = std::vec::IntoIter<usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Tour {
    type Item = &'a usize;

    type IntoIter = core::slice::Iter<'a, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Index<usize> for Tour {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Tour {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
