use crate::{
    criteria::CapacityInput,
    insert_neighborhood::{
        criteria::capacity::iter::CapacityMoves, neighborhood::InsertNeighborhood,
    },
    tsp::Tsp,
};
use orx_local_search::{EvalMove, MoveGenerator, Problem};
use orx_meta::queue::{NonEmptyQueue, Single};

#[derive(Default)]
pub struct CapacityMoveGenerator;

impl<'i> MoveGenerator<'i> for CapacityMoveGenerator {
    type Problem = Tsp;

    type Neighborhood = InsertNeighborhood;

    type Input = Single<&'i CapacityInput>;

    fn moves<'a>(
        &'a mut self,
        input: Self::Input,
        tour: &'a <Self::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a
    where
        'i: 'a,
    {
        CapacityMoves::new(input.front(), tour)
    }
}
