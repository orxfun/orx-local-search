use crate::{
    composed_criteria::ComposedCriteria,
    criterion::{CandidateMoveOf, Criterion, InputOf, ObjectOf},
    move_generator::MoveGenerator,
    sorted_intersecting_iterator::SortedIntersectingIter,
};

pub struct ComposedMoveGenerator<X1, X2>(X1::MoveGenerator, X2::MoveGenerator)
where
    X1: Criterion,
    X2: Criterion<Problem = X1::Problem>;

impl<X1, X2> ComposedMoveGenerator<X1, X2>
where
    X1: Criterion,
    X2: Criterion<Problem = X1::Problem>,
{
    pub fn new(move_generator1: X1::MoveGenerator, move_generator2: X2::MoveGenerator) -> Self {
        Self(move_generator1, move_generator2)
    }
}

impl<X1, X2> MoveGenerator for ComposedMoveGenerator<X1, X2>
where
    X1: Criterion,
    X2: Criterion<Problem = X1::Problem>,
{
    type X = ComposedCriteria<X1, X2>;

    fn moves<'a>(
        &mut self,
        object: &ObjectOf<Self::X>,
        (input1, input2): InputOf<'a, Self::X>,
    ) -> impl Iterator<Item = CandidateMoveOf<Self::X>> {
        let moves1 = self.0.moves(object, input1);
        let moves2 = self.1.moves(object, input2);
        SortedIntersectingIter::new(moves1, moves2)
    }
}
