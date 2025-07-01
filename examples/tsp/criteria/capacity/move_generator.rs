use crate::criteria::capacity::{criterion::Capacity, iter::CapacityMoves};
use orx_local_search::{CandidateMoveOf, InputOf, MoveGenerator, SolutionOf};

pub struct CapacityMoveGenerator;

impl MoveGenerator for CapacityMoveGenerator {
    type X = Capacity;

    fn moves<'a>(
        &'a mut self,
        tour: &'a SolutionOf<Self::X>,
        duration_matrix: &'a InputOf<Self::X>,
    ) -> impl Iterator<Item = CandidateMoveOf<Self::X>> + 'a {
        CapacityMoves::new(tour, duration_matrix)
    }
}
