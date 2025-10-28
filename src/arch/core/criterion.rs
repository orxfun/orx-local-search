use super::{eval_soln::EvalSoln, problem::Problem};

pub trait Criterion<P>
where
    P: Problem,
{
    type Input<'i>;

    fn evaluate(input: &Self::Input<'_>, solution: &P::Solution) -> EvalSoln<P>;
}
