use crate::{problem::Problem, symbolic::Symbolic};
use orx_meta::queue::Queue;

pub trait Criterion: Symbolic {
    type Problem: Problem;

    type Input<'i>: Queue + Copy;

    fn evaluate(
        self,
        input: Self::Input<'_>,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> Option<<Self::Problem as Problem>::ObjectiveUnit>;
}
