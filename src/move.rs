use std::fmt::Debug;

pub trait Move: Ord + Eq + Debug {
    type Solution;

    fn apply(&self, solution: Self::Solution) -> Self::Solution;
}
