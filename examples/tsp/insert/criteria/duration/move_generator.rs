use crate::insert::{
    criteria::duration::{DurationMatrix, iter::DurationMoves},
    neighborhood::InsertNeighborhood,
};
use orx_local_search::{CandidateMoveOf, MoveGenerator, Neighborhood, Problem};

pub struct DurationMoveGenerator;

impl<'i> MoveGenerator<'i> for DurationMoveGenerator {
    type Neighborhood = InsertNeighborhood;

    type Input = &'i DurationMatrix;

    fn moves<'a>(
        &'a mut self,
        tour: &'a <<Self::Neighborhood as Neighborhood>::Problem as Problem>::Solution,
        input: &'a Self::Input,
    ) -> impl Iterator<Item = CandidateMoveOf<Self::Neighborhood>> + 'a {
        DurationMoves::new(tour, input)
    }
}
