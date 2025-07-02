use crate::{
    criterion::Criterion,
    move_generator::MoveGenerator,
    problem::{CandidateMoveOf, Problem},
    sorted_intersecting_iterator::SortedIntersectingIter,
};

pub struct ComposedMoveGenerator<'i, X1, X2>(X1::MoveGenerator<'i>, X2::MoveGenerator<'i>)
where
    X1: Criterion,
    X2: Criterion<Problem = X1::Problem>;

impl<'i, X1, X2> ComposedMoveGenerator<'i, X1, X2>
where
    X1: Criterion,
    X2: Criterion<Problem = X1::Problem>,
{
    pub fn new(
        move_generator1: X1::MoveGenerator<'i>,
        move_generator2: X2::MoveGenerator<'i>,
    ) -> Self {
        Self(move_generator1, move_generator2)
    }
}

impl<'i, X1, X2> MoveGenerator<'i> for ComposedMoveGenerator<'i, X1, X2>
where
    X1: Criterion,
    X2: Criterion<Problem = X1::Problem>,
{
    type Problem = X1::Problem;

    type Input = (X1::Input, X2::Input);

    fn moves<'a>(
        &'a mut self,
        solution: &'a <Self::Problem as Problem>::Solution,
        (input1, input2): &'a Self::Input,
    ) -> impl Iterator<Item = CandidateMoveOf<Self::Problem>> + 'a {
        let moves1 = self.0.moves(solution, input1);
        let moves2 = self.1.moves(solution, input2);
        SortedIntersectingIter::new(moves1, moves2)
    }
}
