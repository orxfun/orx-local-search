use crate::{
    Problem, criterion::Criterion, empty_criterion::move_generator::NeighborhoodGenerator,
};
use std::marker::PhantomData;

pub struct EmptyCriterion<P: Problem>(PhantomData<P>);

impl<P: Problem> Default for EmptyCriterion<P> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<P: Problem> Criterion for EmptyCriterion<P> {
    type Problem = P;

    type Input<'a> = ();

    type MoveGenerator = NeighborhoodGenerator<P>;

    fn move_generator(&self) -> Self::MoveGenerator {
        NeighborhoodGenerator::default()
    }
}
