use std::fmt::Debug;

pub trait Move: Ord + Eq + Debug {
    type Solution: Clone;

    fn apply(&self, solution: Self::Solution) -> Self::Solution;
}
