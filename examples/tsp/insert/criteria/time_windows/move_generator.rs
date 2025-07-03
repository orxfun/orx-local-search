use crate::insert::{
    criteria::time_windows::{TimeWindowInput, iter::TimeWindowMoves},
    neighborhood::InsertNeighborhood,
};
use orx_local_search::{CandidateMoveOf, MoveGenerator, Neighborhood, Problem};

pub struct TimeWindowMoveGenerator;

impl<'i> MoveGenerator<'i> for TimeWindowMoveGenerator {
    type Neighborhood = InsertNeighborhood;

    type Input = &'i TimeWindowInput<'i>;

    fn moves<'a>(
        &'a mut self,
        tour: &'a <<Self::Neighborhood as Neighborhood>::Problem as Problem>::Solution,
        input: &'a Self::Input,
    ) -> impl Iterator<Item = CandidateMoveOf<Self::Neighborhood>> + 'a {
        TimeWindowMoves::new(tour, input)
    }
}
