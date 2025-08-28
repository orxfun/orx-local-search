use crate::{crit::Criterion, eval_soln::EvalSoln, objective::Objective, problem::Problem};
use core::marker::PhantomData;
use orx_meta::queue::EmptyQueue;

#[derive(Clone, Copy)]
pub struct EmptyCriterion<P: Problem>(PhantomData<P>);

impl<P: Problem> Default for EmptyCriterion<P> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<P: Problem> Criterion for EmptyCriterion<P> {
    type Problem = P;

    type Input<'i> = EmptyQueue;

    fn evaluate(
        _: Self::Input<'_>,
        _: &<Self::Problem as Problem>::Solution,
    ) -> EvalSoln<Self::Problem> {
        EvalSoln::Feasible(<<Self::Problem as Problem>::Objective as Objective>::identity())
    }
}
