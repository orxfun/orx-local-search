use crate::{eval_soln::EvalSoln, problem::Problem, symbolic::Symbolic};
use orx_meta::queue::Queue;

pub trait Criteria: Symbolic {
    type Problem: Problem;

    type Input<'i>: Queue + Copy;

    fn evaluate(
        input: Self::Input<'_>,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> EvalSoln<Self::Problem>;
}
