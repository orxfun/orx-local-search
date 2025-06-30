use crate::{
    CandidateMove, MoveGenerator, ObjectiveValue, Problem,
    criterion::{CandidateMoveOf, Criterion, InputOf, SolutionOf},
};
use std::{convert::identity, marker::PhantomData};

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

pub struct NeighborhoodGenerator<P: Problem>(PhantomData<P>);

impl<P: Problem> Default for NeighborhoodGenerator<P> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<P: Problem> MoveGenerator for NeighborhoodGenerator<P> {
    type X = EmptyCriterion<P>;

    fn moves<'a>(
        &mut self,
        solution: &SolutionOf<Self::X>,
        _: InputOf<'a, Self::X>,
    ) -> impl Iterator<Item = CandidateMoveOf<Self::X>> {
        P::neighborhood(solution)
            .map(|mv| CandidateMove::new(mv, <P::ObjectiveValue as ObjectiveValue>::identity()))
    }
}
