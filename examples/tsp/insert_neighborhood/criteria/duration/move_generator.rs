use crate::{
    criteria::DurationMatrix,
    insert_neighborhood::{
        criteria::duration::iter::DurationMoves, neighborhood::InsertNeighborhood,
    },
    tsp::Tsp,
};
use orx_local_search::{EvalMove, MoveGenerator, Problem};

#[derive(Default)]
pub struct DurationMoveGenerator;

impl<'i> MoveGenerator<'i> for DurationMoveGenerator {
    type Problem = Tsp;

    type Neighborhood = InsertNeighborhood;

    type Input = DurationMatrix;

    fn moves<'a>(
        &'a mut self,
        input: &'a Self::Input,
        tour: &'a <Self::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a {
        DurationMoves::new(input, tour)
    }
}
