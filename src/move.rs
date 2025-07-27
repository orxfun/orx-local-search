use crate::problem::Problem;

pub trait Move: Ord + Eq {
    type Problem: Problem;

    fn apply(
        &self,
        solution: <Self::Problem as Problem>::Solution,
    ) -> <Self::Problem as Problem>::Solution;
}
