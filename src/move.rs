use crate::problem::Problem;
use core::fmt::Debug;

pub trait Move: Ord + Eq + Debug {
    type Problem: Problem;

    fn apply(
        &self,
        solution: <Self::Problem as Problem>::Solution,
    ) -> <Self::Problem as Problem>::Solution;
}
