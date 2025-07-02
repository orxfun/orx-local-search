use crate::{
    ObjectiveValue, Problem, SolutionOf,
    criterion::{Criterion, ObjectiveUnitOf},
    empty_criterion::move_generator::NeighborhoodGenerator,
};
use orx_meta::queue::Empty;
use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct EmptyCriterion<P: Problem>(PhantomData<P>);

impl<P: Problem> Default for EmptyCriterion<P> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<P: Problem> Criterion for EmptyCriterion<P> {
    type Problem = P;

    type Input<'i> = ();

    type MoveGenerator<'i> = NeighborhoodGenerator<P>;

    type InputQueue = Empty;

    type ComposeWith<X>
        = X
    where
        X: Criterion<Problem = Self::Problem>;

    fn move_generator<'i>(self) -> Self::MoveGenerator<'i> {
        NeighborhoodGenerator::default()
    }

    fn evaluate(self, _: &SolutionOf<Self>, _: &Self::Input<'_>) -> Option<ObjectiveUnitOf<Self>> {
        Some(<P::ObjectiveValue as ObjectiveValue>::identity())
    }
}
