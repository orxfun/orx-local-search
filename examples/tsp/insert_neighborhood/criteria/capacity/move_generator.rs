use crate::{
    criteria::CapacityInput,
    insert_neighborhood::{
        criteria::capacity::iter::CapacityMoves, neighborhood::InsertNeighborhood,
    },
    tsp::Tsp,
};
use orx_local_search::{EvalMove, MoveGenerator, Problem};

#[derive(Default)]
pub struct CapacityMoveGenerator;

impl<'i> MoveGenerator<'i> for CapacityMoveGenerator {
    type Problem = Tsp;

    type Neighborhood = InsertNeighborhood;

    type Input = CapacityInput;

    fn moves<'a>(
        &'a mut self,
        tour: &'a <Self::Problem as Problem>::Solution,
        input: &'a Self::Input,
    ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a {
        CapacityMoves::new(tour, input)
    }
}
