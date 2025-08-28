use crate::{Criterion, Objective, Problem};
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
        self,
        _: Self::Input<'_>,
        _: &<Self::Problem as Problem>::Solution,
    ) -> Option<<Self::Problem as Problem>::ObjectiveUnit> {
        Some(<P::Objective as Objective>::identity())
    }
}
