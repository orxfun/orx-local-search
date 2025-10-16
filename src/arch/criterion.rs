use crate::{eval_soln::EvalSoln, inputs::InputsQueue, problem::Problem};

pub trait Criterion<P>
where
    P: Problem,
{
    type Input<'i>: InputsQueue;

    fn evaluate(input: &Self::Input<'_>, solution: &P::Solution) -> EvalSoln<P>;
}
