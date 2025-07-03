use crate::{
    Neighborhood, criterion::Criterion, move_generator::MoveGenerator, problem::Problem,
    sorted_intersecting_iterator::SortedIntersectingIter,
};

pub struct ComposedMoveGenerator<'i, X1, X2>(X1::MoveGenerator<'i>, X2::MoveGenerator<'i>)
where
    X1: Criterion,
    X2: Criterion<Neighborhood = X1::Neighborhood>;

impl<'i, X1, X2> ComposedMoveGenerator<'i, X1, X2>
where
    X1: Criterion,
    X2: Criterion<Neighborhood = X1::Neighborhood>,
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
    X2: Criterion<Neighborhood = X1::Neighborhood>,
{
    type Neighborhood = X1::Neighborhood;

    type Input = (X1::Input<'i>, X2::Input<'i>);

    fn moves<'a>(
        &'a mut self,
        solution: &'a <<Self::Neighborhood as Neighborhood>::Problem as Problem>::Solution,
        (input1, input2): &'a Self::Input,
    ) -> impl Iterator<Item = crate::CandidateMoveOf<Self::Neighborhood>> + 'a {
        let moves1 = self.0.moves(solution, input1);
        let moves2 = self.1.moves(solution, input2);
        SortedIntersectingIter::new(moves1, moves2)
    }
}
