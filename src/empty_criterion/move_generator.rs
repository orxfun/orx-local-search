use crate::{
    CandidateMove, MoveGenerator, ObjectiveValue, Problem,
    criterion::{CandidateMoveOf, InputOf, SolutionOf},
    empty_criterion::criterion::EmptyCriterion,
};
use std::marker::PhantomData;

pub struct NeighborhoodGenerator<P: Problem>(PhantomData<P>);

impl<P: Problem> Default for NeighborhoodGenerator<P> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<P: Problem> MoveGenerator for NeighborhoodGenerator<P> {
    type X = EmptyCriterion<P>;

    fn moves<'a, 'b, 'c>(
        &'a mut self,
        solution: &'b SolutionOf<Self::X>,
        _: InputOf<'c, Self::X>,
    ) -> impl Iterator<Item = CandidateMoveOf<Self::X>> + 'a + 'b + 'c {
        P::neighborhood(solution).map(move |mv| {
            CandidateMove::new(mv, <P::ObjectiveValue as ObjectiveValue>::identity())
        })
    }
}
