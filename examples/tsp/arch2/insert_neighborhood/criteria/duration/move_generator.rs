use crate::{
    criteria::DurationMatrix,
    insert_neighborhood::{
        criteria::duration::iter::DurationMoves, neighborhood::InsertNeighborhood,
    },
    tsp::Tsp,
};
use orx_local_search::{EvalMove, MoveGenerator, Problem};
use orx_meta::queue::{NonEmptyQueue, Single};

#[derive(Default)]
pub struct DurationMoveGenerator;

impl<'i> MoveGenerator<'i> for DurationMoveGenerator {
    type Problem = Tsp;

    type Neighborhood = InsertNeighborhood;

    type Input = Single<&'i DurationMatrix>;

    fn moves<'a>(
        &'a mut self,
        input: Self::Input,
        tour: &'a <Self::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a
    where
        'i: 'a,
    {
        DurationMoves::new(input.front(), tour)
    }
}
