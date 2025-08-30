use crate::{eval_soln::EvalSoln, problem::Problem};

pub trait Criterion {
    type Problem: Problem;

    type Input<'i>: Copy;

    fn evaluate(
        input: Self::Input<'_>,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> EvalSoln<Self::Problem>;
}
