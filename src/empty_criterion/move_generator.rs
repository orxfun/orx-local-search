use crate::{CandidateMove, CandidateMoveOf, MoveGenerator, ObjectiveValue, Problem};
use std::marker::PhantomData;

pub struct NeighborhoodGenerator<P: Problem>(PhantomData<P>);

impl<P: Problem> Default for NeighborhoodGenerator<P> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<P: Problem> MoveGenerator<'_> for NeighborhoodGenerator<P> {
    type Problem = P;

    type Input = ();

    fn moves<'a>(
        &'a mut self,
        solution: &'a <Self::Problem as Problem>::Solution,
        _: &'a Self::Input,
    ) -> impl Iterator<Item = CandidateMoveOf<Self::Problem>> + 'a {
        P::neighborhood(solution).map(move |mv| {
            CandidateMove::new(mv, <P::ObjectiveValue as ObjectiveValue>::identity())
        })
    }
}
