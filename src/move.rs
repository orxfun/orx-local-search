use crate::problem::Problem;
use core::fmt::Debug;

pub trait Move: Ord + Eq + Debug {
    type Problem: Problem;

    fn apply(&self, solution: &mut <Self::Problem as Problem>::Solution);
}
