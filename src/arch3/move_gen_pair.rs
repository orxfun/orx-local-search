use crate::{
    Criterion, CriterionOnNeighborhood, move_gen::MoveGenerator,
    sorted_intersecting_iter::SortedIntersectingIter,
};
use orx_meta::queue::{NonEmptyQueue, Pair};

pub struct ComposedMoveGenerator<'i, X1, X2>(X1::MoveGenerator<'i>, X2::MoveGenerator<'i>)
where
    X1: CriterionOnNeighborhood,
    X2: CriterionOnNeighborhood<Problem = X1::Problem, Neighborhood = X1::Neighborhood>,
    X2::Criterion: Criterion<Problem = <X1::Criterion as Criterion>::Problem>; // TODO: do we need the last constraint

impl<'i, X1, X2> Default for ComposedMoveGenerator<'i, X1, X2>
where
    X1: CriterionOnNeighborhood,
    X2: CriterionOnNeighborhood<Problem = X1::Problem, Neighborhood = X1::Neighborhood>,
    X2::Criterion: Criterion<Problem = <X1::Criterion as Criterion>::Problem>,
{
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}

impl<'i, X1, X2> MoveGenerator<'i> for ComposedMoveGenerator<'i, X1, X2>
where
    X1: CriterionOnNeighborhood,
    X2: CriterionOnNeighborhood<Problem = X1::Problem, Neighborhood = X1::Neighborhood>,
    X2::Criterion: Criterion<Problem = <X1::Criterion as Criterion>::Problem>,
{
    type Problem = X1::Problem;

    type Neighborhood = X1::Neighborhood;

    type Input = Pair<InputOf<'i, X1>, InputOf<'i, X2>>;

    fn moves<'a>(
        &'a mut self,
        input: Self::Input,
        solution: &'a <Self::Problem as crate::Problem>::Solution,
    ) -> impl Iterator<Item = crate::EvalMove<Self::Neighborhood>> + 'a
    where
        'i: 'a,
    {
        let (input1, input2) = input.pop_front();
        let moves1 = self.0.moves(input1, solution);
        let moves2 = self.1.moves(input2, solution);
        SortedIntersectingIter::new(moves1, moves2)
    }
}

type InputOf<'i, X> = <<X as CriterionOnNeighborhood>::Criterion as Criterion>::Input<'i>;
