use crate::{
    criteria::CapacityInput,
    insert::{criteria::capacity::iter::CapacityMoves, neighborhood::InsertNeighborhood},
};
use orx_local_search::{CandidateMoveOf, MoveGenerator, Neighborhood, Problem};

pub struct CapacityMoveGenerator;

impl<'i> MoveGenerator<'i> for CapacityMoveGenerator {
    type Neighborhood = InsertNeighborhood;

    type Input = &'i CapacityInput;

    fn moves<'a>(
        &'a mut self,
        tour: &'a <<Self::Neighborhood as Neighborhood>::Problem as Problem>::Solution,
        input: &'a Self::Input,
    ) -> impl Iterator<Item = CandidateMoveOf<Self::Neighborhood>> + 'a {
        CapacityMoves::new(tour, input)
    }
}
