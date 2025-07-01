use crate::{
    ObjectiveValue, Problem, SolutionOf,
    criterion::{Criterion, ObjectiveUnitOf},
    empty_criterion::move_generator::NeighborhoodGenerator,
};
use orx_meta::queue::Empty;
use std::marker::PhantomData;

pub struct EmptyCriterion<P: Problem>(PhantomData<P>);

impl<P: Problem> Default for EmptyCriterion<P> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<P: Problem> Criterion for EmptyCriterion<P> {
    type Problem = P;

    type Input = ();

    type InputQueue = Empty;

    type MoveGenerator = NeighborhoodGenerator<P>;

    fn move_generator() -> Self::MoveGenerator {
        NeighborhoodGenerator::default()
    }

    fn evaluate(_: &SolutionOf<Self>, _: &Self::Input) -> Option<ObjectiveUnitOf<Self>> {
        Some(<P::ObjectiveValue as ObjectiveValue>::identity())
    }
}
