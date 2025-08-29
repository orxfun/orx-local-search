use crate::{Criteria, eval_soln::EvalSoln, problem::Problem, symbolic::Symbolic};
use orx_meta::queue::{NonEmptyQueue, Single};

pub trait Criterion: Symbolic {
    type Problem: Problem;

    type Input<'i>: Copy;

    fn evaluate(
        input: Self::Input<'_>,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> EvalSoln<Self::Problem>;
}
