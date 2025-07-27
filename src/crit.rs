use crate::{problem::Problem, symbolic::Symbolic};

pub trait Criterion: Symbolic {
    type Problem: Problem;

    type Input<'i>;

    fn evaluate(
        self,
        input: &Self::Input<'_>,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> Option<<Self::Problem as Problem>::ObjectiveUnit>;
}
