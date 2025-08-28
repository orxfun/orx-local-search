use crate::{Criterion, eval_soln::EvalSoln, problem::Problem, symbolic::Symbolic};
use orx_meta::queue::Queue;

pub trait Criteria: Symbolic {
    type Problem: Problem;

    type Input<'i>: Queue;

    // type PushBackInput<'i, X2>: Queue + Copy
    // where
    //     X2: Criterion<Problem = Self::Problem>;

    fn evaluate(
        input: Self::Input<'_>,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> EvalSoln<Self::Problem>;
}
