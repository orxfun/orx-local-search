use crate::tsp::Tsp;
use orx_local_search::{Move, Problem};
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Insert {
    pub current_position: usize,
    pub target_position: usize,
}

impl Insert {
    pub fn new(current_position: usize, target_position: usize) -> Self {
        Self {
            current_position,
            target_position,
        }
    }
}

impl Move for Insert {
    type Problem = Tsp;

    fn apply(
        &self,
        mut tour: <Self::Problem as Problem>::Solution,
    ) -> <Self::Problem as Problem>::Solution {
        match self.current_position.cmp(&self.target_position) {
            Ordering::Equal => {}
            Ordering::Less => {
                let current = tour[self.current_position];
                for p in self.current_position..self.target_position {
                    tour[p] = tour[p + 1];
                }
                tour[self.target_position] = current;
            }
            Ordering::Greater => {
                let current = tour[self.current_position];
                for p in (self.target_position..self.current_position).rev() {
                    tour[p + 1] = tour[p];
                }
                tour[self.target_position] = current;
            }
        }
        tour
    }
}
