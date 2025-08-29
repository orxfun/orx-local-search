use crate::{eval_soln::EvalSoln, problem::Problem, symbolic::Symbolic};

pub trait Criteria: Symbolic {
    type Problem: Problem;

    type Input<'i>: Copy;

    // type PushBackInput<'i, X2>: Queue + Copy
    // where
    //     X2: Criterion<Problem = Self::Problem>;

    // type Compose<Y>: Criteria<Problem = Self::Problem>
    // where
    //     Y: Criteria<Problem = Self::Problem>;

    fn evaluate(
        input: Self::Input<'_>,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> EvalSoln<Self::Problem>;
}
